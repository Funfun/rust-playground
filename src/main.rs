use std::io; // listed, without we would call `std::io::stdid()`.

fn main() {
    println!("Guess the number!");

    println!("Please enter your guess.");

    let mut guess = String::new(); //  let - def variable, mu - makes it mutable, String is stdlib type that is growable, UTF-8 encoded bit of text. :: - accessor to assoc methods of `type`.

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line"); // io::stdin() => instance of io::Stdin, returns `io::Result` => `Ok` or `Err`

    println!("You guessed: {}", guess);

    for (i, ch) in guess.chars().enumerate() {
       println!("Index: {}, char: {}", i, ch);
    }
}
