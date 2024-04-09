// see toml file too
// import the standard input/output(io) lib
use std::io;

fn main(){
    println!("Guess the number!");

    println!("Please input your guess.");

    // creates a mutable variable of the type String
    let mut guess = String::new();

    // In Rust variables ares immutable by default, i.e. 
    // constants in value 
    // mut marks the variable as mutable

    // :: indicates the access of the Type

    io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read line");

    // :: in this we accessed the module io
    // stdin() -> standard input method
    // read_line(&param) -> stores read line in param
    // &var -> indicates the reference of the var
    // references are also immutable, thus the &mut
    // expect -> error handler

    println!("You guessed: {guess}");

    // {var} prints the var value
}