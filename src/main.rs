mod token;
mod lexer;
mod repl;

fn main() {
    let user = std::env::var("USER").unwrap();
    println!("Hello {}! This is the Monkey programming language!", user);
    println!("Feel free to type in commands");
    repl::start();
}
