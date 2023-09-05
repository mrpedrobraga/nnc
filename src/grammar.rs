use lazy_regex::*;

pub static TOKEN_MATCHERS: &'static [TokenMatcher] = &[
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

pub static NANO_PARSE_RULES: &'static [(&'static str, &[ParseRule])] = &[
    (
        "Program",
        &[
            ParseRule::SingleToken(TokenName::IntLiteral),
            ParseRule::SingleToken(TokenName::Newline),
            ParseRule::SingleToken(TokenName::EOF),
        ],
    ),
    ("Expr", &[ParseRule::SingleToken(TokenName::IntLiteral)]),
    ("Literal", &[ParseRule::SingleToken(TokenName::IntLiteral)]),
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

#[derive(Debug)]
pub struct Token {
    pub name: TokenName,
    pub occurrence_index: usize,
    pub length: usize,
}

#[derive(Debug)]
pub struct TokenMatcher {
    pub regex: &'static Lazy<Regex>,
    pub name: TokenName,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TokenName {
    EOF,

    // In nano, these tokens are 'ghost' tokens,
    // that is, they are by default ignorable by parse rules.
    // Ghost tokens are *still* matcheable in parse rules...
    Indent,       // '\n\t{x}' where x > current_indentation
    Newline,      // '\n'
    Whitespace,   // ' '
    BlockComment, // '### .*? ###'
    Comment,      // '#'

    Identifier, // hello foo_bar Baz ❤️

    ThinArrow, // '->'
    Pipe,      // '|>'
    Semicolon, // ';'
    Comma,     // ','

    ScopeAnnotation,  // '%%test'
    BranchAnnotation, // '#%define'

    IntLiteral,     // '42', '0xFA', '0b0110_1100'
    StringLiteral,  // '".*?"'
    BooleanLiteral, // 'true' | 'false' | 'yes' | 'no' | '0b' | '1b'

    ParenthesisOpen,     // '('
    ParenthesisClose,    // ')'
    SqBracketsOpen,      // '['
    SqBracketsClose,     // ']'
    CrBracketsOpen,      // '{'
    CrBracketsClose,     // '}'
    AgBracketsOpen,      // '<'
    AgBracketsClose,     // '>'
    Reticences,          // '...'
    ExclusiveReticences, // '..'

    Colon, // ':'

    OpAddrof, // addrof
    OpTypeof, // typeof
    OpType,   // type
    OpValue,  // value

    OpIs,                 // 'is'
    OpXis,                // 'xis'
    OpAnd,                // 'and'
    OpOr,                 // 'or'
    OpNot,                // 'not'
    OpPipe,               // '|'
    OpAmpersand,          // '&'
    OpPlus,               // '+'
    OpDash,               // '-'
    OpAsterisk,           // '*'
    OpForwardSlash,       // '/'
    OpDoubleForwardSlash, // '//'
    OpPercent,            // %
    OpEqSign,             // =
}

#[derive(Debug)]
pub struct ASTNode<'a> {
    pub matched_with: &'static str,
    pub content: Vec<ASTNodeContent<'a>>,
}

#[derive(Debug)]
pub enum ASTNodeContent<'a> {
    Leaf(&'a Token),
    Branches(Box<ASTNode<'a>>),
}

#[derive(Debug)]
pub struct AST<'a> {
    pub is_abstract: bool,
    pub root: ASTNode<'a>,
}

pub fn get_rule<'a>(
    list: &'a [(&'static str, &[ParseRule])],
    key: &str,
) -> Option<&'a [ParseRule]> {
    let r = NANO_PARSE_RULES.iter().position(|(_k, _v)| *_k == key);

    let (_, v) = match r {
        None => return None,
        Some(i) => list[i],
    };

    return Some(v);
}

#[derive(Debug)]
pub enum ParseRule {
    SingleToken(TokenName),
    Keyword(TokenName, String),
    Disjunction(&'static [ParseRule]),
    Conjunction(&'static [ParseRule]),
    Nest(&'static str),
}
