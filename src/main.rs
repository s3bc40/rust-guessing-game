use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // Var immutable by default
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failes to read line");

    println!("You guessed: {}", guess);
}
