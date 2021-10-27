use std::io::{self, Write};

fn main() {
    loop {
        print!("db > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input.starts_with(".") {
            match parse_meta_command(&input) {
                Ok(MetaCommand::Exit) => break,
                Err(_) => {
                    println!("Unrecognized command: {}", input);
                    continue
                }
            }
        }
    }
}

enum MetaCommand {
    Exit
}

fn parse_meta_command(input: &str) -> Result<MetaCommand, ()> {
    match input {
        ".exit" => Ok(MetaCommand::Exit),
        _ => Err(())
    }
}
