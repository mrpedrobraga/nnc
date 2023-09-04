use colored::Colorize;

use crate::grammar::{TOKEN_MATCHERS, Token, TokenName};

/*
    Tokenizer, which will be used both by the compiler,
    the formatter, the linter and the LSP.
*/
pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut char_offset: usize = 0;

    let mut column: usize = 0;
    let mut line: usize = 0;

    while char_offset < source.len() {
        let mut has_match = false;

        'matchers:
        for matcher in TOKEN_MATCHERS {
            let source_slice: &str = &source[char_offset..];
            let captures = matcher.regex.captures(source_slice);

            let captures = match captures {
                None => continue 'matchers,
                Some(c) => c
            };

            let matched_string = &captures[0];

            match matcher.name {
                | TokenName::BlockComment
                | TokenName::StringLiteral => {
                    for c in matched_string.chars() {
                        match c {
                            '\n' => {column = 0; line += 1},
                            _ => {column += 1;}
                        }
                    }
                }
                TokenName::Newline => {column = 0; line += 1}
                _ => {column += matched_string.len()}
            }
            
            tokens.push(Token {name: matcher.name, occurrence_index: char_offset, length: matched_string.len()});
            char_offset += matched_string.len();
            has_match = true;
        }

        if has_match {
        } else {
            println!("{} : Unexpected token '{}' at Ln {}, Col {}.", "FAILED".red(), &source.chars().nth(char_offset).unwrap(), line, column);
            break;
        }
    }

    return tokens
} 

pub fn build_tree(source: &[Token]) {
    match source[..] {
        [ Token {name: TokenName::Identifier, ..} ] => {}
        _ => {}
    }

    println!("{:#?}", source);
}