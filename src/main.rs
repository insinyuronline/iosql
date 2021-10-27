use std::io::{self, Write};

fn main() {
    loop {
        print!("db > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().eq(".exit") {
            break;
        } else {
            println!("Your command was: {}", input);
        }
    }
}
