use colored::Colorize;

use crate::grammar::{
    get_rule, is_ghost_token, ASTNode, ASTNodeContent, ParseRule, Token, TokenName, AST,
    NANO_PARSE_RULES, TOKEN_MATCHERS,
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

        'matchers: for matcher in TOKEN_MATCHERS {
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

    return tokens;
}

/// Builds a tree given a pool of vectors, and a starting rule.
pub fn build_tree<'a>(
    source: &'a Vec<Token>,
    start_rule_name: &'static str,
    keep_ghost_tokens: bool,
) -> Option<AST<'a>> {
    let start_rule = get_rule(NANO_PARSE_RULES, start_rule_name);
    match start_rule {
        None => return None,
        Some(r) => r,
    };

    let tree = match_rule(
        &source[..],
        start_rule_name,
        ParseContext {
            parse_rule_list: NANO_PARSE_RULES,
        },
        keep_ghost_tokens,
    );

    let tree = match tree {
        None => return None,
        Some(t) => t,
    };

    return Some(AST {
        is_abstract: !keep_ghost_tokens,
        root: ASTNode {
            matched_with: start_rule_name,
            content: tree.content,
        },
    });
}

// Matches a rule to the beggining of a slice of the token pool.
pub fn match_rule<'a>(
    slice: &'a [Token],
    rule_name: &str,
    context: ParseContext,
    _keep_ghost_tokens: bool,
) -> Option<ParseRuleMatchResult<'a>> {
    // Retrieves the rule from the list to match
    // The list should probably rather be stored
    // in the ParseContext.
    let rule = get_rule(context.parse_rule_list, rule_name);
    let rule = match rule {
        None => {
            println!("{} '{}'.", "No rule found with the name", rule_name);
            return None;
        }
        Some(r) => r,
    };

    // Matching the rule
    match rule {
        // Single token o/ IDENTIFIER /
        ParseRule::SingleToken(tok) => {
            println!("{:?}", tok);
            if slice[0].name == *tok {
                return Some(ParseRuleMatchResult {
                    matched: true,
                    advance: 1,
                    content: ASTNodeContent::Leaf(&slice[0..1]),
                });
            } else if is_ghost_token(tok) {
                return Some(ParseRuleMatchResult {
                    matched: false,
                    advance: 1,
                    content: ASTNodeContent::Leaf(&slice[0..1]),
                });
            } else {
                return None;
            }
        }

        // Single token o/ IDENTIFIER & {content: "while"} /
        ParseRule::Keyword(_tok, _contentt) => None,

        // Disjunction o/ ( SEMICOLON | NEWLINE ) /
        ParseRule::Disjunction(_cases) => None,

        // Conjunction o/ SEMICOLON & NEWLINE /
        // I think it's mostly unused in this parser
        ParseRule::Conjunction(_cases) => None,

        // Reference to another ParseRule --
        // it's what makes this a recursive descent parser
        ParseRule::Nest(_sub_rule) => None,
    }
}

pub struct ParseContext<'a> {
    pub parse_rule_list: &'a [(&'static str, &'a ParseRule)],
}

pub struct ParseRuleMatchResult<'a> {
    pub matched: bool,
    pub advance: usize,

    pub content: ASTNodeContent<'a>,
}
