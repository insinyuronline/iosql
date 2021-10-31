use std::error::Error;
use std::io::{self, Write};

mod table;
use table::Table;

mod parser;
use parser::meta_command::MetaCommand;
use parser::statement::Statement;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut table = Table::new();

    loop {
        print!("db > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input.starts_with(".") {
            match MetaCommand::parse(&input) {
                Ok(MetaCommand::Exit) => break,
                Err(_) => {
                    println!("Unrecognized command: {}", input);
                    continue
                }
            }
        }

        let statement = match Statement::parse(&input) {
            Ok(v) => v,
            Err(_) => {
                println!("Unrecognized command: {}", input);
                continue
            },
        };

        statement.execute(&mut table).unwrap();
    }

    Ok(())
}
