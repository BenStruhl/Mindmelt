#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // special tokens that are used by the compiler not actual code
    ILLEGAL,
    EOF,

    // list of all applicable tokens
    INT(i64), 
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