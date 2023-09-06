use colored::Colorize;

use crate::{
    grammar::{
        get_rule, get_token_string_content, ASTNode, ASTNodeContent, ParseRule, Token, TokenName,
        AST,
    },
    nano_grammar::NANO_TOKEN_RULES,
    nano_grammar::{is_ghost_token, NANO_PARSE_RULES},
};

/// Tokenizer, which will be used both by the compiler,
/// the formatter, the linter and the LSP.
pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut char_offset: usize = 0;

    let mut column: usize = 0;
    let mut line: usize = 0;

    while char_offset < source.len() {
        let mut has_match = false;

        'matchers: for matcher in NANO_TOKEN_RULES {
            let source_slice: &str = &source[char_offset..];
            let captures = matcher.regex.captures(source_slice);

            let captures = match captures {
                None => continue 'matchers,
                Some(c) => c,
            };

            let matched_string = &captures[0];

            match matcher.name {
                TokenName::BlockComment | TokenName::StringLiteral => {
                    for c in matched_string.chars() {
                        match c {
                            '\n' => {
                                column = 0;
                                line += 1
                            }
                            _ => {
                                column += 1;
                            }
                        }
                    }
                }
                TokenName::Newline => {
                    column = 0;
                    line += 1
                }
                _ => column += matched_string.len(),
            }

            tokens.push(Token {
                name: matcher.name,
                occurrence_index: char_offset,
                length: matched_string.len(),
            });
            char_offset += matched_string.len();
            has_match = true;
        }

        if has_match {
        } else {
            println!(
                "{} : Unexpected token '{}' at Ln {}, Col {}.",
                "FAILED".red(),
                &source.chars().nth(char_offset).unwrap(),
                line,
                column
            );
            break;
        }
    }

    tokens.push(Token {
        name: TokenName::EOF,
        occurrence_index: char_offset,
        length: 0,
    });

    return tokens;
}

/// Builds a tree given a pool of vectors, and a starting rule.
pub fn build_tree<'a>(
    source_str: &'a str,
    source: &'a Vec<Token>,
    top_level_rule_name: &'static str,
    keep_ghost_tokens: bool,
) -> Option<AST<'a>> {
    let top_level_rule = get_rule(NANO_PARSE_RULES, top_level_rule_name);
    let top_level_rule = match top_level_rule {
        None => return None,
        Some(r) => r,
    };

    let tree = match_rule(
        &source[..],
        top_level_rule,
        &ParseContext {
            source_string: source_str,
            parse_rule_list: NANO_PARSE_RULES,
        },
        keep_ghost_tokens,
    );

    let tree = match tree {
        None => {
            println!("Parsing stage failed.");
            return None;
        }
        Some(t) => t,
    };

    return Some(AST {
        is_abstract: !keep_ghost_tokens,
        root: ASTNode {
            matched_with: top_level_rule_name,
            content: tree.content,
        },
    });
}

// Matches a rule to the beggining of a slice of the token pool.
pub fn match_rule<'a>(
    source_token_pool: &'a [Token],
    rule: &[ParseRule],
    context: &ParseContext,
    keep_ghost_tokens: bool,
) -> Option<ParseRuleMatchResult<'a>> {
    let mut token_slice_offset = 0;
    let mut fragment_index = 0;
    let mut content: Vec<ASTNodeContent> = Vec::new();

    // For each fragment
    while fragment_index < rule.len() {
        let fragment = &rule[fragment_index];

        if token_slice_offset >= source_token_pool.len() {
            println!("Ran out of tokens.");
            break;
        }

        // Matching the fragments one by one
        match fragment {
            // Single token o/ IDENTIFIER /
            // Optionally matching the token's content as well
            ParseRule::SingleToken(rule_token_name, rule_token_content) => {
                let current_source_token = &source_token_pool[token_slice_offset];
                let token_content_matches = match rule_token_content {
                    None => true,
                    Some(s) => {
                        get_token_string_content(current_source_token, context.source_string) == *s
                    }
                };

                // Source token matches Rule token!
                if current_source_token.name == *rule_token_name && token_content_matches {
                    content.push(ASTNodeContent::Tok(&source_token_pool[token_slice_offset]));
                    fragment_index += 1;
                    token_slice_offset += 1;
                // Source token is a ghost token (ignorable)
                } else if is_ghost_token(&current_source_token.name) {
                    if keep_ghost_tokens {
                        content.push(ASTNodeContent::Tok(&source_token_pool[token_slice_offset]))
                    }
                    token_slice_offset += 1;
                // Neither, rule can not be accepted
                } else {
                    return None;
                }
            }

            ParseRule::Optional(sub_fragments) => {
                let sub_match =
                    match_rule(source_token_pool, sub_fragments, context, keep_ghost_tokens);

                match sub_match {
                    None => content.push(ASTNodeContent::None),
                    Some(t) => {
                        content.push(ASTNodeContent::Grouping(t.content));

                        token_slice_offset += t.advance;
                    }
                }

                fragment_index += 1;
            }

            ParseRule::Many(sub_fragments) => {
                let mut matched_at_least_once = false;
                let mut many_content: Vec<ASTNodeContent> = Vec::new();

                loop {
                    let sub_match = match_rule(
                        &source_token_pool[token_slice_offset..],
                        sub_fragments,
                        context,
                        keep_ghost_tokens,
                    );

                    match sub_match {
                        Some(t) => {
                            many_content.push(ASTNodeContent::Grouping(t.content));
                            println!("Matching");
                            token_slice_offset += t.advance;
                            matched_at_least_once = true;
                        }
                        None => {
                            break;
                        }
                    }
                }

                if !matched_at_least_once {
                    return None;
                }

                fragment_index += 1;
                content.push(ASTNodeContent::Grouping(many_content));
            }

            // Disjunction o/ ( SEMICOLON | NEWLINE ) /
            ParseRule::Disjunction(cases) => {
                let mut matched_any = false;
                for case in *cases {
                    let nested_match = match_rule(
                        &source_token_pool[token_slice_offset..],
                        case,
                        &context,
                        keep_ghost_tokens,
                    );
                    let nested_match = match nested_match {
                        None => continue,
                        Some(m) => m,
                    };

                    fragment_index += 1;
                    token_slice_offset += nested_match.advance;

                    content.push(ASTNodeContent::Grouping(nested_match.content));
                    matched_any = true;
                }

                if !matched_any {
                    return None;
                }
            }

            // Conjunction o/ SEMICOLON & NEWLINE /
            // I think it's mostly unused in this parser
            ParseRule::Conjunction(cases) => {
                let mut failed_any = false;
                // Similar to how short-circuiting works, the last
                // case of a conjunction is the one whose match is chosen,
                // in a subtle breach of Conjunction Comutativity.
                let mut last_match: Option<ParseRuleMatchResult> = None;
                for case in *cases {
                    let nested_match = match_rule(
                        &source_token_pool[token_slice_offset..],
                        case,
                        &context,
                        keep_ghost_tokens,
                    );
                    let nested_match = match nested_match {
                        None => {
                            failed_any = true;
                            break;
                        }
                        Some(m) => m,
                    };
                    last_match = Some(nested_match)
                }

                if failed_any {
                    return None;
                }

                match last_match {
                    None => return None,
                    Some(lm) => {
                        fragment_index += 1;
                        token_slice_offset += lm.advance;

                        content.push(ASTNodeContent::Grouping(lm.content));
                    }
                }
            }

            // Reference to another ParseRule --
            // it's what makes this a recursive descent parser
            ParseRule::Nest(sub_rule_name) => {
                let sub_rule = get_rule(context.parse_rule_list, &sub_rule_name);
                let sub_rule = match sub_rule {
                    None => return None,
                    Some(r) => r,
                };

                let nested_match = match_rule(
                    &source_token_pool[token_slice_offset..],
                    sub_rule,
                    &context,
                    keep_ghost_tokens,
                );
                let nested_match = match nested_match {
                    None => return None,
                    Some(nm) => nm,
                };

                fragment_index += 1;
                token_slice_offset += nested_match.advance;

                content.push(ASTNodeContent::Node(ASTNode {
                    matched_with: sub_rule_name,
                    content: nested_match.content,
                }))
            }
        };
    }

    return Some(ParseRuleMatchResult {
        matched: true,
        advance: token_slice_offset,
        content,
    });
}

pub struct ParseContext<'a> {
    pub source_string: &'a str,
    pub parse_rule_list: &'a [(&'static str, &'a [ParseRule<'a>])],
}

pub struct ParseRuleMatchResult<'a> {
    pub matched: bool,
    pub advance: usize,

    pub content: Vec<ASTNodeContent<'a>>,
}
