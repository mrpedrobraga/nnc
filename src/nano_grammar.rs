use crate::grammar::{ParseRule, TokenMatcher, TokenName};
use lazy_regex::regex as rx;

// The rules used to create the AST building blocks
pub static NANO_TOKEN_RULES: &'static [TokenMatcher] = &[
    // Whitespace
    TokenMatcher {
        name: TokenName::Newline,
        regex: rx!(r"^\r?\n"),
    },
    TokenMatcher {
        name: TokenName::Whitespace,
        regex: rx!(r"^[ \s\r\f\t]+"),
    },
    // AST Operators
    TokenMatcher {
        name: TokenName::Comma,
        regex: rx!(r"^,"),
    },
    TokenMatcher {
        name: TokenName::Semicolon,
        regex: rx!(r"^;"),
    },
    TokenMatcher {
        name: TokenName::Colon,
        regex: rx!(r"^:"),
    },
    TokenMatcher {
        name: TokenName::ThinArrow,
        regex: rx!(r"^->"),
    },
    TokenMatcher {
        name: TokenName::Pipe,
        regex: rx!(r"^\|>"),
    },
    TokenMatcher {
        name: TokenName::ParenthesisOpen,
        regex: rx!(r"^\("),
    },
    TokenMatcher {
        name: TokenName::ParenthesisClose,
        regex: rx!(r"^\)"),
    },
    TokenMatcher {
        name: TokenName::SqBracketsOpen,
        regex: rx!(r"^\["),
    },
    TokenMatcher {
        name: TokenName::SqBracketsClose,
        regex: rx!(r"^\]"),
    },
    TokenMatcher {
        name: TokenName::CrBracketsOpen,
        regex: rx!(r"^\{"),
    },
    TokenMatcher {
        name: TokenName::CrBracketsClose,
        regex: rx!(r"^\}"),
    },
    // Literals
    TokenMatcher {
        name: TokenName::IntLiteral,
        regex: rx!(r"^(0x[0-9a-zA-Z_]+|0b[0-9]+|[0-9_]+)"),
    },
    TokenMatcher {
        name: TokenName::StringLiteral,
        regex: rx!(r#"^".*?""#),
    },
    // Identifier / Keyword
    TokenMatcher {
        name: TokenName::Identifier,
        regex: rx!(r"^[a-zA-Z_][a-zA-Z0-9_]*"),
    },
    // Comments
    TokenMatcher {
        name: TokenName::Comment,
        regex: rx!(r"^###[\s\S]*?###"),
    },
    TokenMatcher {
        name: TokenName::Comment,
        regex: rx!(r"^#[\s\S]*?\n"),
    },
    // Operators
    TokenMatcher {
        name: TokenName::OpAnd,
        regex: rx!(r"^#.*?\n"),
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
pub static NANO_PARSE_RULES: &'static [(&'static str, &[ParseRule])] = &[
    ("Program", &[ParseRule::Nest("Exprs")]),
    (
        "Exprs",
        &[
            ParseRule::Nest("Expr"),
            ParseRule::OptionalMany(&[
                ParseRule::SingleToken(TokenName::Semicolon, None),
                ParseRule::Nest("Expr"),
            ]),
        ],
    ),
    (
        "Expr",
        &[ParseRule::SingleToken(TokenName::IntLiteral, None)],
    ),
];

pub static NANO_AI_RULES: &'static [(&'static str, fn(i32) -> i32)] = &[("Program", |i| i)];
