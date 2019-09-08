use crate::token::Token;
use std::io::stdin;
#[derive(Debug)]
pub struct Interpreter {
    pub program: Vec<u8>,
    pub index: usize,
} 

impl Interpreter {
    pub fn new(number: usize) -> Interpreter {
        return Interpreter {
            program: vec![0; number],
            index: 0,
        }
    }

    pub fn find_closing_paren(&mut self, tokens: &Vec<Token>, index: usize) -> usize {
        let mut close_pos = index;
        let mut counter = 1;
        while counter > 0 {
            close_pos += 1;
            if close_pos > tokens.len() {
                panic!("did not have closing paren for loop started at positon {}", index + 1);
            }
            let t = tokens[..][close_pos];
            if t == Token::LSBRACE {
                counter += 1;
            } else if t == Token::RSBRACE {
                counter -= 1
            }
        }
        return close_pos + 1;
    }  
    
    pub fn find_open_paren(&mut self, tokens: &Vec<Token>, index: usize) -> usize {
        let mut open_pos = index;
        let mut counter = 1;
        while counter > 0 {
            open_pos -= 1;
            if open_pos > tokens.len() {
                panic!("did not have opening paren for loop started at positon {}", index + 1);
            }
            let t = tokens[..][open_pos];
            if t == Token::LSBRACE {
                counter -= 1;
            } else if t == Token::RSBRACE {
                counter += 1
            }
        }
        return open_pos;
    } 
    
        
    
    pub fn run_commands(&mut self, tokens: Vec<Token>) -> isize {
        let mut i = 0;
        let mut switched_i = false;
        while i < tokens.len() {
            match tokens[i] {
                Token::LT => { self.index -= 1 },
                Token::GT => { self.index += 1 },
                Token::DOT => {  println!("{:#?} \n head at cell: {} \n", self.program, self.index)},
                Token::MINUS => { self.program[self.index] -= 1 },
                Token::COMMA => {
                        let mut s = String::new();
                        stdin().read_line(&mut s).expect("Did not enter a correct string value");
                        if s.len() == 1 && s.is_ascii() {
                            let read_char: u8 = s.parse().unwrap();
                            self.program[self.index] = read_char;
                        } else {
                            panic!("given text is not a valid charcter");
                        }
                } ,
                Token::PLUS => { self.program[self.index] += 1 },
                Token::LSBRACE => {
                   if self.program[self.index] == 0 {
                       i = self.find_closing_paren(&tokens, i);
                       switched_i = true;
                   } else {

                   }
                },
                Token::PERCENT => panic!("this feature is not implemented yet"),
                Token::BANG => panic!("this feature is not implemented yet"),
                Token::RSBRACE => {
                       i = self.find_open_paren(&tokens, i);
                       switched_i = true;
                },
                Token::EOF => return 1,
                Token::ILLEGAL => panic!("Illegal Token"), 
    
            }
            if !switched_i {
                i += 1;
            }
            switched_i = false;
        }
        return 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter::Interpreter;
    #[test]
    fn test_sanity() {
        assert_eq!(1 + 1, 2);
    }

}