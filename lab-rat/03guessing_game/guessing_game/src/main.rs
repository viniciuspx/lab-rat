// see toml file too
// import the standard input/output(io) lib
use std::{cmp::Ordering, io};

// in rand lib we use rng that generates random numbers
use rand::Rng;

fn main() {
    // generates a whole random number between 0 ~ 100
    let secret_number = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Guess the number!");
        // creates a mutable variable of the type String
        let mut guess = String::new();

        // In Rust variables are immutable by default, i.e.
        // constants in value
        // mut marks the variable as mutable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // :: in this we accessed the module io
        // stdin() -> standard input method
        // read_line(&param) -> stores read line in param
        // &var -> indicates the reference of the var
        // references are also immutable, thus the &mut
        // expect -> error handler

        // We "shadow" (shadowing) the last guess var with the new u32 one
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // trim() -> removes whitespaces and \n
        // parse() -> parses the string to the annotated type in this case after the ":" -> u32

        println!("You guessed: {guess}");
        // println!({var}) prints the var value

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
