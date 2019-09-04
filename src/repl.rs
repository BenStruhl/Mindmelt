use crate::token::Token;
use crate::lexer::Lexer;
use crate::interpreter::Interpreter;

use std::io;
use std::process;
use std::io::prelude::*;


const PROMPT: &'static str = ">> ";


pub fn start() {
        let mut inter = Interpreter::new(5);
        loop {
            print!("{}", PROMPT);
            io::stdout().flush().expect("REPLError: failed to flush stdin");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("REPLError: failed to readline");
            let output: Result<String, _> = input.parse();
            let repl_code = match output {
                Ok(x) => x,
                Err(error) => panic!("REPLError: {}", error),
            };
            if repl_code.trim() == "quit" {
                return ();
            }
            let mut lex = Lexer::new(repl_code.as_str());
            let mut tok = Token::ILLEGAL;
            let mut vec_tok: Vec<Token> = Vec::new();
            while tok != Token::EOF {
                tok = lex.next_token();
                vec_tok.push(tok);
            }
            inter.run_commands(vec_tok);
        }
}
#[cfg(test)]
mod tests {
    use crate::repl;

    #[test]
    fn test_sanity() {
        repl::start();
        assert_eq!(1 + 1, 2);
    }

}