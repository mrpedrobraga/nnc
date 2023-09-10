use lazy_regex::*;

#[derive(Debug)]
pub struct Token<'a> {
    pub name: TokenName,
    pub str_content: Option<&'a str>,
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
    None,
    Tok(&'a Token<'a>),
    Grouping(Vec<ASTNodeContent<'a>>),
    Node(ASTNode<'a>),
}

#[derive(Debug)]
pub struct AST<'a> {
    pub is_abstract: bool,
    pub root: ASTNode<'a>,
}

pub fn get_rule<'a>(
    list: &'a [(&'static str, &[ParseRule])],
    key: &str,
) -> Option<&'a [ParseRule<'a>]> {
    let r = list.iter().position(|(_k, _v)| *_k == key);

    let (_, v) = match r {
        None => return None,
        Some(i) => list[i],
    };

    return Some(v);
}

#[derive(Debug)]
pub enum ParseRule<'a> {
    SingleToken(TokenName, Option<&'a str>),
    Disjunction(&'a [&'a [ParseRule<'a>]]),
    Conjunction(&'a [&'a [ParseRule<'a>]]),
    Nest(&'static str),
    Optional(&'a [ParseRule<'a>]),
    Many(&'a [ParseRule<'a>]),
    OptionalMany(&'a [ParseRule<'a>]),
}
