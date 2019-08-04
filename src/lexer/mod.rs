use token::Token; 

#[allow(dead_code)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: u8,
}
#[allow(dead_code)]
impl<'a> Lexer<'a> {
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    } 
    pub fn new(input: &'a str) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        return l;
    }

    fn read_number(&mut self) -> Token {
        let position = self.position;
        while (self.ch as char).is_ascii_digit() {
            self.read_char();
        }
        self.read_position -= 1;
        return Token::INT(self.input[position..self.position].parse::<i64>().unwrap());
    }
    
    fn skip_whitespace(&mut self) {
        while (self.ch as char).is_ascii_whitespace() {
            self.read_char()
        }
    }

    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input.as_bytes()[self.read_position];
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok: Token= match self.ch as char {
            '0' ... '9' => self.read_number(),
            '<'   => Token::LT,
            '>'   => Token::GT,
            '.'   => Token::DOT,
            '-'   => Token::MINUS,
            ','   => Token::COMMA,
            '+'   => Token::PLUS,
            '['   => Token::LSBRACE,
            '%'   => Token::PERCENT,
            ']'   => Token::RSBRACE,
            '\0'  => Token::EOF,
             _    => Token::ILLEGAL, 

        };

        self.read_char();

        if tok == Token::ILLEGAL {
          panic!("cannot match given token to langauge syntax: {}", self.ch)
        };
        return tok;
    }
}
#[cfg(test)]
mod tests {
    use token::Token; 
    use lexer::Lexer;
    #[test]
    fn test_next_token() {
        let input = r#"30000++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.%,"#;
        let test_array = vec![
                    Token::INT(30000),
                    Token::PLUS,
                    Token::PLUS,
                    Token::PLUS,
                    Token::PLUS,
                    Token::PLUS,
                    Token::PLUS,
                    Token::PLUS,
                    Token::PLUS,
                    Token::LSBRACE,
                    Token::GT,
                    Token::PLUS,
                    Token::PLUS,
                    Token::PLUS,
                    Token::PLUS,
                    Token::LSBRACE,
                    Token::GT,
                    Token::PLUS,
                    Token::PLUS,
                    Token::GT,
                    Token::PLUS,
                    Token::PLUS,
                    Token::PLUS,
                    Token::GT,
                    Token::PLUS,
                    Token::PLUS,
                    Token::PLUS,
                    Token::GT,
                    Token::PLUS,
                    Token::LT,
                    Token::LT,
                    Token::LT,
                    Token::LT,
                    Token::MINUS,
                    Token::RSBRACE,
                    Token::GT,
                    Token::PLUS,
                    Token::GT,
                    Token::PLUS,
                    Token::GT,
                    Token::MINUS,
                    Token::GT,
                    Token::GT,
                    Token::PLUS,
                    Token::LSBRACE,
                    Token::LT,
                    Token::RSBRACE,
                    Token::LT,
                    Token::MINUS,
                    Token::RSBRACE,
                    Token::GT,
                    Token::GT,
                    Token::DOT,
                    Token::GT,
                    Token::MINUS, 
                    Token::MINUS, 
                    Token::MINUS,
                    Token::DOT, 
                    Token::PLUS,
                    Token::PLUS,
                    Token::PLUS,
                    Token::PLUS,
                    Token::PLUS,
                    Token::PLUS,
                    Token::PLUS,
                    Token::DOT,
                    Token::DOT,
                    Token::PLUS,
                    Token::PLUS,
                    Token::PLUS,
                    Token::DOT,
                    Token::GT,
                    Token::GT,
                    Token::DOT,
                    Token::LT,
                    Token::MINUS,
                    Token::DOT,
                    Token::LT,
                    Token::DOT,
                    Token::PLUS,
                    Token::PLUS,
                    Token::PLUS,
                    Token::DOT,
                    Token::MINUS,
                    Token::MINUS,
                    Token::MINUS,
                    Token::MINUS,
                    Token::MINUS,
                    Token::MINUS,
                    Token::DOT,
                    Token::MINUS,
                    Token::MINUS,
                    Token::MINUS,
                    Token::MINUS,
                    Token::MINUS,
                    Token::MINUS,
                    Token::MINUS,
                    Token::MINUS,
                    Token::DOT,
                    Token::GT,
                    Token::GT,
                    Token::PLUS,
                    Token::DOT,
                    Token::GT,
                    Token::PLUS,
                    Token::PLUS,
                    Token::DOT,
                    Token::PERCENT,
                    Token::COMMA,
               ];
        let mut lexerr = Lexer::new(&input);
        for tt in test_array.iter() {
            let tok = lexerr.next_token();
            assert_eq!(tok, *tt);
    