use std::io;

fn main() {
    println!("Enter a number");
    let mut guessed_number = String::new();
    io::stdin()
        .read_line(&mut guessed_number)
        .expect("Failed to read line");

    println!("{guessed_number}");

    let mut i = 0;

    let mut guessed_number: String = guessed_number.trim();

    /*while i < guessed_number {

    }*/
}
