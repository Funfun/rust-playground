use std::io; // listed, without we would call `std::io::stdid()`.

enum Message {
    Publish,
    Draft,
    Delete,
    Write(String)
}
// https://habr.com/post/430294

fn inspect(message: Message) {
    match message {
        Message::Publish => println!("publishing"),
        Message::Delete => println!("deleting"),
        Message::Draft => println!("drafting"),
        Message::Write(text) => {
            println!("printing");
            let mut copy_of_text = &text;
            sendText(&mut copy_of_text);
        }
    }
}

fn sendText(text:&mut String) {
    // TODO: API call
    text.truncate(160);
    println!("Brief {}", text);
}


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
