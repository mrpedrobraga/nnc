use lazy_regex::*;

#[derive(Debug)]
pub struct Token {
    pub name: TokenName,
    pub occurrence_index: usize,
    pub length: usize
}

#[derive(Debug)]
pub struct TokenMatcher {
    pub regex: &'static Lazy<Regex>,
    pub name: TokenName,
}

pub static TOKEN_MATCHERS: &'static [TokenMatcher] = &[
    // Make sure to escape everything well :'-)
    TokenMatcher { name: TokenName::Newline, regex: regex!(r"^\n") },
    TokenMatcher { name: TokenName::Whitespace, regex: regex!(r"^[ \s\r\f\t\n]+") },

    TokenMatcher { name: TokenName::Comma, regex: regex!(r"^,") },
    TokenMatcher { name: TokenName::Semicolon, regex: regex!(r"^;") },
    TokenMatcher { name: TokenName::Colon, regex: regex!(r"^:") },
    TokenMatcher { name: TokenName::ThinArrow, regex: regex!(r"^->") },
    TokenMatcher { name: TokenName::Pipe, regex: regex!(r"^\|>") },

    TokenMatcher { name: TokenName::ParenthesisOpen, regex: regex!(r"^\(") },
    TokenMatcher { name: TokenName::ParenthesisClose, regex: regex!(r"^\)") },
    TokenMatcher { name: TokenName::SqBracketsOpen, regex: regex!(r"^\[") },
    TokenMatcher { name: TokenName::SqBracketsClose, regex: regex!(r"^\]") },
    TokenMatcher { name: TokenName::CrBracketsOpen, regex: regex!(r"^\{") },
    TokenMatcher { name: TokenName::CrBracketsClose, regex: regex!(r"^\}") },

    TokenMatcher { name: TokenName::IntLiteral, regex: regex!(r"^(0x[0-9a-zA-Z_]+|0b[0-9]+|[0-9_]+)") },

    TokenMatcher { name: TokenName::Identifier, regex: regex!(r"^[a-zA-Z_][a-zA-Z0-9_]*") },

    TokenMatcher { name: TokenName::Comment, regex: regex!(r"^###[\s\S]*?###") },
    TokenMatcher { name: TokenName::OpAnd, regex: regex!(r"^#.*?\n") },

    TokenMatcher { name: TokenName::StringLiteral, regex: regex!(r#"^".*?""#) },
];

#[derive(Debug, Clone, Copy)]
pub enum TokenName {
    // In nano, these tokens are 'ghost' tokens,
    // that is, they are by default ignorable by parse rules.
    // Ghost tokens are *still* matcheable in parse rules...
    Indent,         // '\n\t{x}' where x > current_indentation
    Newline,        // '\n'
    Whitespace,     // ' '
    BlockComment,  // '### .*? ###'
    Comment,        // '#'

    Identifier,     // hello foo_bar Baz ❤️

    ThinArrow,  // '->'
    Pipe,  // '|>'
    Semicolon,  // ';'
    Comma,      // ','

    ScopeAnnotation,   // '%%test'
    BranchAnnotation,  // '#%define'

    IntLiteral,        // '42', '0xFA', '0b0110_1100'
    StringLiteral,     // '".*?"'
    BooleanLiteral,    // 'true' | 'false' | 'yes' | 'no' | '0b' | '1b'

    ParenthesisOpen,   // '('
    ParenthesisClose,  // ')'
    SqBracketsOpen,   // '['
    SqBracketsClose,  // ']'
    CrBracketsOpen,   // '{'
    CrBracketsClose,  // '}'
    AgBracketsOpen,   // '<'
    AgBracketsClose,  // '>'
    Reticences,             // '...'
    ExclusiveReticences,   // '..'

    Colon,     // ':'

    OpAddrof, // addrof
    OpTypeof, // typeof
    OpType,   // type
    OpValue,  // value

    OpIs,  // 'is'
    OpXis, // 'xis'
    OpAnd, // 'and'
    OpOr,  // 'or'
    OpNot, // 'not'
    OpPipe,        // '|'
    OpAmpersand,   // '&'
    OpPlus,        // '+'
    OpDash,        // '-'
    OpAsterisk,    // '*'
    OpForwardSlash,        // '/'
    OpDoubleForwardSlash, // '//'
    OpPercent,     // %
    OpEqSign,     // =
}

pub struct CSTNode {
    matched_with: ParseRuleName,
    branches: Vec<CSTNode>
}

pub struct ASTNode {
    matched_with: ParseRuleName,
    branches: Vec<CSTNode>
}

pub enum ParseRuleName {
    Program,

    ExprListSemicolon,
    ExprListComma,

    RecursiveExpr,
    Expr,

    BinaryOp,
    UnaryOp,

    SingleToken(Token),
}