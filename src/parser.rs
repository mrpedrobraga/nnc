use colored::Colorize;

use crate::grammar::{
    get_rule, ASTNode, ParseRule, ParseRuleName, Token, TokenName, AST, NANO_PARSE_RULES,
    TOKEN_MATCHERS,
};

/*
    Tokenizer, which will be used both by the compiler,
    the formatter, the linter and the LSP.
*/
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

pub fn build_tree(source: &Vec<Token>, keep_ghost_tokens: bool) -> Option<AST> {
    let tree = match_rule(
        &source[..],
        "Literal",
        ParseContext { tok_index: 0 },
        keep_ghost_tokens,
    );

    let tree = match tree {
        None => return None,
        Some(t) => t,
    };

    return Some(AST {
        is_abstract: !keep_ghost_tokens,
        root: ASTNode {
            matched_with: ParseRuleName::SingleToken(Token {
                name: TokenName::IntLiteral,
                occurrence_index: 0,
                length: 1,
            }),
            branches: None,
            leaf: tree.leaf,
        },
    });
}

pub fn match_rule<'a>(
    slice: &'a [Token],
    rule_name: &str,
    context: ParseContext,
    keep_ghost_tokens: bool,
) -> Option<ParseRuleMatchResult<'a>> {
    let rule = get_rule(NANO_PARSE_RULES, rule_name);
    let rule = match rule {
        None => {
            println!("{} '{}'.", "No rule found with the name", rule_name);
            return None;
        }
        Some(r) => r,
    };

    match rule {
        ParseRule::SingleToken(tok) => {
            println!("{:?}", tok);
            if slice[0].name == *tok {
                return Some(ParseRuleMatchResult {
                    matched: true,
                    advance: 1,
                    branches: None,
                    leaf: Some(&slice[0..1]),
                });
            } else {
                return None;
            }
        }
        ParseRule::Keyword(tok, content) => None,
        ParseRule::Disjunction(cases) => None,
        ParseRule::Conjunction(cases) => None,
        ParseRule::Nest(sub_rule) => None,
    }
}

pub struct ParseContext {
    pub tok_index: usize,
}

pub struct ParseRuleMatchResult<'a> {
    pub matched: bool,
    pub advance: usize,

    pub leaf: Option<&'a [Token]>,
    pub branches: Option<ASTNode<'a>>,
}
