use crate::token::Token;
use std::process;
use std::io::stdin;

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
    pub fn run_commands(&mut self, tokens: Vec<Token>) -> isize {
        let mut i = 0;
        while i < tokens.len() {
            println!("{:#?}", tokens[i]);
            println!("{}", i);
            match tokens[i] {
                Token::LT => { self.index -= 1 },
                Token::GT => { self.index += 1 },
                Token::DOT => {  print!("{}", (self.program[self.index] as char))},
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
                       let mut brace_count = 0;
                       let mut y = i;
                       while(y >= 0) {
                           if(tokens[y] == Token::LSBRACE) {
                               brace_count += 1;
                           }
                           y -= 1;
                       }
                         let mut x = tokens.len() - 1;
                         while x >= 0 {
                            if tokens[x] == Token::RSBRACE && brace_count == 1 {
                                i = x + 1;
                                println!("new i {}", i);
                                break;
                            }
                            if tokens[x] == Token::RSBRACE {
                                brace_count -= 1;
                            }
                            x -= 1; 
                        }
                    }
                },
                Token::PERCENT => panic!("this feature is not implemented yet"),
                Token::BANG => panic!("this feature is not implemented yet"),
                Token::RSBRACE => {
                       let mut brace_count = 0;
                       let mut y = i;
                       while(y < tokens.len()) {
                           if(tokens[y] == Token::RSBRACE) {
                               brace_count += 1;
                           }
                           y += 1;
                       }
                       let mut x: usize = i;
                        while x >= 0 { 
                            if tokens[x] == Token::LSBRACE && brace_count == 1 {
                                i = x;
                                break;
                            }
                            if tokens[x] == Token::LSBRACE {
                                brace_count -= 1;
                            }
                            x -= 1;
                        }
                },
                Token::EOF => return 0,
                Token::ILLEGAL => panic!("Illegal Token"), 
    
            }
        i += 1;
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