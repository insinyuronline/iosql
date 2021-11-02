use std::error::Error;
use std::io::{self, Write};

mod table;
use table::Table;

mod parser;
use parser::meta_command::MetaCommand;
use parser::statement::Statement;

pub fn run() -> Result<(), Box<dyn Error>> {
    let db_input = "user
name|email|postal_code
gio|gio@insinyur.online|12345
iqbal|iqbal@insinyur.online|12345
EOF";
    let mut table = Table::deserialize(db_input.to_string());

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
                Ok(MetaCommand::Print) => {
                    println!("{}", table.serialize());
                    continue;
                },
                Err(_) => {
                    println!("Unrecognized command: {}", input);
                    continue;
                }
            }
        }

        let statement = match Statement::parse(&input) {
            Ok(v) => v,
            Err(_) => {
                println!("Unrecognized command: {}", input);
                continue;
            }
        };

        statement.execute(&mut table).unwrap();
    }

    Ok(())
}
