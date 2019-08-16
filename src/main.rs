pub mod lexer;
pub mod token;
pub mod repl;
pub mod interpreter;

fn main() {
    println!("Welcome to the Mindmelt Programming Language!");
    println!("feel free to type in commands");
    loop {
        repl::start();
    }
}