use std::collections::HashMap;

use crate::table::{ColumnData, Table};

pub enum Statement {
    INSERT,
    SELECT,
}

impl Statement {
    pub fn parse(input: &str) -> Result<Statement, ()> {
        match input.to_lowercase().split_whitespace().next() {
            Some("insert") => Ok(Statement::INSERT),
            Some("select") => Ok(Statement::SELECT),
            _ => Err(())
        }
    }

    pub fn execute(&self, table: &mut Table) -> Result<(), ()> {
        match self {
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
}
