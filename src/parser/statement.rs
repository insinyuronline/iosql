use std::collections::HashMap;

use crate::table::{ColumnData, Table};

pub enum Statement {
    Insert { insert_data: Vec<ColumnData> },
    Select { select_columns: Vec<String> },
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
                Ok(Statement::Insert { insert_data })
            }
            Some("select") => {
                let select_columns: Vec<String> = tokenized_input.map(|x| x.to_string()).collect();
                Ok(Statement::Select { select_columns })
            }
            _ => Err(()),
        }
    }

    pub fn execute(&self, table: &mut Table) -> Result<(), ()> {
        match self {
            Statement::Insert { insert_data } => {
                let mut row: HashMap<String, ColumnData> = HashMap::new();
                for (i, column) in table.headers.iter().enumerate() {
                    row.insert(
                        column.to_string(),
                        match insert_data.get(i) {
                            Some(v) => v.clone(),
                            None => ColumnData::Varchar("NULL".to_string()),
                        },
                    );
                }
                table.rows.push(row);
                Ok(())
            }
            Statement::Select { select_columns } => {
                for row in &table.rows {
                    let mut output: Vec<String> = Vec::new();
                    for column in select_columns {
                        match row.get(column) {
                            Some(v) => output.push(v.to_string()),
                            None => output.push("NULL".to_string()),
                        };
                    }
                    println!("{}", output.join(" "));
                }
                Ok(())
            }
        }
    }
}
