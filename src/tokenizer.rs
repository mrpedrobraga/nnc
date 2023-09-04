use crate::grammar::{TOKEN_MATCHERS, Token, TokenName};

/*
    Tokenizer, which will be used both by the compiler,
    the formatter, the linter and the LSP.
*/
pub fn tokenize(source: &str) {
    let mut tokens: Vec<Token> = vec![];
    let mut char_offset: usize = 0;

    while char_offset < source.len() {
        let mut has_match = false;

        for matcher in TOKEN_MATCHERS {
            let source_slice: &str = &source[char_offset..];
            let captures = matcher.regex.captures(source_slice);

            let captures = match captures {
                None => continue,
                Some(c) => c
            };

            let matched_string = &captures[0];
            
            tokens.push(Token {name: matcher.name, occurrence_index: char_offset, length: matched_string.len()});
            char_offset += matched_string.len();
            has_match = true;
        }

        if has_match {
        } else {
            println!("Unexpected token '{}' at {}.", &source.chars().nth(char_offset).unwrap(), char_offset);
            break;
        }
    }

    for tok in tokens {
        match tok.name {
            TokenName::Whitespace => {},
            _ => println!("[{:?} : '{}']", tok.name, &source[tok.occurrence_index..tok.occurrence_index+tok.length]),
        }
    }
} 