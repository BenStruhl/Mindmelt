#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    // special tokens that are used by the compiler not actual code
    ILLEGAL,
    EOF,

    // list of all applicable tokens
    PLUS,
    MINUS,
    BANG,
    PERCENT,
    LT,
    GT,
    COMMA,
    DOT,
    LSBRACE,
    RSBRACE,
 }