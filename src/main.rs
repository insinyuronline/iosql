use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

fn main() {
    let mut table = initialize_table();

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

        let _ = execute_statement(&mut table, statement);
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

pub struct Table {
    pub name: String,
    pub headers: Vec<String>,
    pub rows: Vec<HashMap<String, ColumnData>>,
}

fn initialize_table() -> Table {
    Table {
        name: "user".to_string(), 
        headers: vec![
            "name".to_string(), 
            "email".to_string(), 
            "postal_code".to_string(),
        ], 
        rows: Vec::new(),
    }
}

pub enum ColumnData {
    Int(i32),
    Varchar(String),
}

impl fmt::Display for ColumnData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ColumnData::Int(v) => write!(f, "{}", v),
            ColumnData::Varchar(v) => write!(f, "{}", v),
        }
    }
}
