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

        let statement = match parse_statement(&input) {
            Ok(v) => v,
            Err(_) => {
                println!("Unrecognized command: {}", input);
                continue
            },
        };

        let _ = execute_statement(statement);
    }
}

enum MetaCommand {
    Exit
}

enum Statement {
    INSERT,
    SELECT,
}

fn parse_meta_command(input: &str) -> Result<MetaCommand, ()> {
    match input {
        ".exit" => Ok(MetaCommand::Exit),
        _ => Err(())
    }
}

fn parse_statement(input: &str) -> Result<Statement, ()> {
    match input.to_lowercase().split_whitespace().next() {
        Some("insert") => Ok(Statement::INSERT),
        Some("select") => Ok(Statement::SELECT),
        _ => Err(())
    }
}

fn execute_statement(statement: Statement) -> Result<(), ()> {
    match statement {
        Statement::INSERT => {
            println!("Insert!");
            Ok(())
        },
        Statement::SELECT => {
            println!("Select!");
            Ok(())
        }
    }
}