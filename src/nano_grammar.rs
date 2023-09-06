use crate::grammar::{ParseRule, TokenMatcher, TokenName};
use lazy_regex::*;

// The rules used to create the AST building blocks
pub static NANO_TOKEN_RULES: &'static [TokenMatcher] = &[
    // Whitespace
    TokenMatcher {
        name: TokenName::Newline,
        regex: regex!(r"^\r?\n"),
    },
    TokenMatcher {
        name: TokenName::Whitespace,
        regex: regex!(r"^[ \s\r\f\t]+"),
    },
    // AST Operators
    TokenMatcher {
        name: TokenName::Comma,
        regex: regex!(r"^,"),
    },
    TokenMatcher {
        name: TokenName::Semicolon,
        regex: regex!(r"^;"),
    },
    TokenMatcher {
        name: TokenName::Colon,
        regex: regex!(r"^:"),
    },
    TokenMatcher {
        name: TokenName::ThinArrow,
        regex: regex!(r"^->"),
    },
    TokenMatcher {
        name: TokenName::Pipe,
        regex: regex!(r"^\|>"),
    },
    TokenMatcher {
        name: TokenName::ParenthesisOpen,
        regex: regex!(r"^\("),
    },
    TokenMatcher {
        name: TokenName::ParenthesisClose,
        regex: regex!(r"^\)"),
    },
    TokenMatcher {
        name: TokenName::SqBracketsOpen,
        regex: regex!(r"^\["),
    },
    TokenMatcher {
        name: TokenName::SqBracketsClose,
        regex: regex!(r"^\]"),
    },
    TokenMatcher {
        name: TokenName::CrBracketsOpen,
        regex: regex!(r"^\{"),
    },
    TokenMatcher {
        name: TokenName::CrBracketsClose,
        regex: regex!(r"^\}"),
    },
    // Literals
    TokenMatcher {
        name: TokenName::IntLiteral,
        regex: regex!(r"^(0x[0-9a-zA-Z_]+|0b[0-9]+|[0-9_]+)"),
    },
    TokenMatcher {
        name: TokenName::StringLiteral,
        regex: regex!(r#"^".*?""#),
    },
    // Identifier / Keyword
    TokenMatcher {
        name: TokenName::Identifier,
        regex: regex!(r"^[a-zA-Z_][a-zA-Z0-9_]*"),
    },
    // Comments
    TokenMatcher {
        name: TokenName::Comment,
        regex: regex!(r"^###[\s\S]*?###"),
    },
    // Operators
    TokenMatcher {
        name: TokenName::OpAnd,
        regex: regex!(r"^#.*?\n"),
    },
];

pub fn is_ghost_token(tname: &TokenName) -> bool {
    match tname {
        TokenName::Whitespace
        | TokenName::Indent
        | TokenName::Comment
        | TokenName::BlockComment
        | TokenName::Newline => true,
        _ => return false,
    }
}

// The rules used to create the AST
pub static NANO_PARSE_RULES: &'static [(&'static str, &[ParseRule])] = &[(
    "Program",
    &[ParseRule::Many(&[ParseRule::SingleToken(
        TokenName::Identifier,
        None,
    )])],
)];
