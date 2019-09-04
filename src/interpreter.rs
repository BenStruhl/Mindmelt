use crate::token::Token;
use std::process;
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
    pub fn find_matching_brace(&mut self, tokens: &Vec<Token>, isRightBrace: bool, index: usize) -> usize {
        if isRightBrace {
            let mut numberOfLeftBraces = Interpreter::count_lbraces(&tokens[0..index], true);
            let mut indexCopy = index;
            for (i, &item) in tokens[index..].iter().rev().enumerate() {
                if item == Token::RSBRACE {
                    numberOfLeftBraces -= 1;
                } if numberOfLeftBraces == 0 {
                    return i;
                }
            }
            return index;
        } else {
            let mut numberOfRightBraces = Interpreter::count_rbraces(&tokens[index..], true);
            let mut indexCopy = index;
            for (i, &item) in tokens[index..].iter().enumerate() {
                if item == Token::LSBRACE {
                    numberOfRightBraces -= 1;
                } if numberOfRightBraces == 0 {
                    return i;
                }
            }
            return index;
        }
    }
    pub fn count_lbraces(tokens: &[Token], rev: bool) -> usize {
        let mut x: usize = 0;
        let iter = tokens.iter();
        if rev {
            for i in iter.rev() {
                if(*i == Token::LSBRACE) {
                    x += 1;
                } else if (*i == Token::RSBRACE) {
                    break;
                } else {}
            }
        } else {
            for i in iter {
                if(*i == Token::LSBRACE) {
                    x += 1;
                } else if (*i == Token::RSBRACE) {
                    break;
                } else {}
            }
        }
        return x;
    }
    pub fn count_rbraces(tokens: &[Token], rev: bool) -> usize {
        let mut x: usize = 0;
        let iter = tokens.iter();
        if rev {
            for i in iter.rev() {
                if(*i == Token::RSBRACE) {
                    x += 1;
                } else if (*i == Token::LSBRACE) {
                    break;
                } else {}
            }
        } else {
            for i in iter {
                if(*i == Token::RSBRACE) {
                    x += 1;
                } else if (*i == Token::LSBRACE) {
                    break;
                } else {}
            }
        }
        return x;
    }
    
        
    
    pub fn run_commands(&mut self, tokens: Vec<Token>) -> isize {
        let mut i = 0;
        while i < tokens.len() {
            println!("{:#?}", tokens[i]);
            println!("{}", i);
            match tokens[i] {
                Token::LT => { self.index -= 1 },
                Token::GT => { self.index += 1 },
                Token::DOT => {  print!("{:#?}", self.program)},
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
                       self.index = self.find_matching_brace(&tokens, true, self.index);
                       println!("THIS IS MY NEW INDEX: {}", self.index);
                   } else {
                       println!("HIHIHI");
                       println!("{}", self.program[self.index]);
                       i += 1;
                       continue;
                   }
                },
                Token::PERCENT => panic!("this feature is not implemented yet"),
                Token::BANG => panic!("this feature is not implemented yet"),
                Token::RSBRACE => {
                       self.index = self.find_matching_brace(&tokens, false, self.index);
                },
                Token::EOF => return 1,
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