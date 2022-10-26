use intern::IString;
use logos::Lexer;
use logos::Logos;

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Represents a lexer token.
pub enum Token {
    #[error]
    #[regex(r"--[^\n]", logos::skip)] // comments
    #[regex(r"[ \t\r\n]", logos::skip)] // whitespace
    /// Represents an error.
    ///
    /// This token will be emitted if no other rules match the input.
    Error,

    #[token("(")]
    /// The `(` token.
    LParen,
    #[token(")")]
    /// The `)` token.
    RParen,

    #[token("{")]
    /// The `{` token.
    LBrack,
    #[token("}")]
    /// The `}` token.
    RBrack,

    #[token(":")]
    /// The `:` token.
    Colon,
    #[token(";")]
    /// The `;` token.
    SemiColon,
    #[token("=")]
    /// The `=` token.
    EqSign,
    #[token(".")]
    /// The `.` token.
    Dot,
    #[token("*")]
    /// The `*` token.
    Star,
    #[token(".*")]
    /// The `.*` token.
    DotStar,
    #[token("&")]
    /// The `&` token.
    Amper,
    #[token(".&")]
    /// The `.&` token.
    DotAmper,

    #[token("fn")]
    /// The `fn` keyword.
    KwdFn,
    #[token("let")]
    /// The `let` keyword.
    KwdLet,
    #[token("mut")]
    /// The `mut` keyword.
    KwdMut,
    #[token("return")]
    /// The `return` keyword.
    KwdRet,

    #[regex(r"[a-zA-Z][_a-zA-Z0-9]*", cb_ident)]
    /// An identifier.
    ///
    /// Identifiers consist of an alphabetical character, followed by any number
    /// of alphanumeric characters or underscores.
    Ident(IString),
}

fn cb_ident(l: &mut Lexer<Token>) -> IString {
    l.slice().into()
}
