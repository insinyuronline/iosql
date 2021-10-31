use std::collections::HashMap;

use crate::table::{ColumnData, Table};

pub enum Statement {
    Insert{insert_data: Vec<ColumnData>},
    Select{select_columns: Vec<String>},
}

impl Statement {
    pub fn parse(input: &str) -> Result<Statement, ()> {
        let tokenized_input = input.to_lowercase();
        let mut tokenized_input = tokenized_input.split_whitespace();

        match tokenized_input.next() {
            Some("insert") => {
                let insert_data: Vec<ColumnData> = tokenized_input
                    .map(|x| ColumnData::parse(x).unwrap())
                    .collect();
                Ok(Statement::Insert{insert_data})
            },
            Some("select") => {
                let select_columns: Vec<String> = tokenized_input
                    .map(|x| x.to_string())
                    .collect();
                Ok(Statement::Select{select_columns})
            },
            _ => Err(())
        }
    }

    pub fn execute(&self, table: &mut Table) -> Result<(), ()> {
        match self {
            Statement::Insert{insert_data: _} => {
                let mut row = HashMap::new();
                row.insert("name".to_string(), ColumnData::Varchar("io".to_string()));
                row.insert("email".to_string(), ColumnData::Varchar("io@insinyur.online".to_string()));
                row.insert("postal_code".to_string(), ColumnData::Int(12345));
                table.rows.push(row);
                Ok(())
            },
            Statement::Select{select_columns: _} => {
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
}
