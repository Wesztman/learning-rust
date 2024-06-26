use std::io;

// Current chapter: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#testing-the-first-part
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
