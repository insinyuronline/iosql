use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

mod table;
use table::ColumnData;
use table::Table;

mod parser;
use parser::meta_command::MetaCommand;

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

        let statement = match parse_statement(&input) {
            Ok(v) => v,
            Err(_) => {
                println!("Unrecognized command: {}", input);
                continue
            },
        };

        let _ = execute_statement(&mut table, statement);
    }

    Ok(())
}
enum Statement {
    INSERT,
    SELECT,
}

fn parse_statement(input: &str) -> Result<Statement, ()> {
    match input.to_lowercase().split_whitespace().next() {
        Some("insert") => Ok(Statement::INSERT),
        Some("select") => Ok(Statement::SELECT),
        _ => Err(())
    }
}

fn execute_statement(table: &mut Table, statement: Statement) -> Result<(), ()> {
    match statement {
        Statement::INSERT => {
            let mut row = HashMap::new();
            row.insert("name".to_string(), ColumnData::Varchar("io".to_string()));
            row.insert("email".to_string(), ColumnData::Varchar("io@insinyur.online".to_string()));
            row.insert("postal_code".to_string(), ColumnData::Int(12345));
            table.rows.push(row);
            Ok(())
        },
        Statement::SELECT => {
            for row in &table.rows {
                print!("{} ", row.get("name").unwrap());
                print!("{} ", row.get("email").unwrap());
                print!("{}", row.get("postal_code").unwrap());
                println!("");
            }
            Ok(())
        }
    }
}
