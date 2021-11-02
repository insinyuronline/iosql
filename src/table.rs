use std::collections::HashMap;
use std::fmt;
use std::fmt::{Write};

pub struct Table {
    pub name: String,
    pub headers: Vec<String>,
    pub rows: Vec<HashMap<String, ColumnData>>,
}

impl Table {
    pub fn _new(name: String, headers: Vec<String>) -> Table {
        Table {
            name,
            headers,
            rows: Vec::new(),
        }
    }

    pub fn serialize(&self) -> String {
        let mut output = String::new();
        write!(output, "{}\n", self.name).unwrap();
        write!(output, "{}\n", self.headers.join("|")).unwrap();
        for (i, row) in self.rows.iter().enumerate() {
            for (j, column) in self.headers.iter().enumerate() {
                if j > 0 {
                    write!(output, "|{}", row.get(column).unwrap()).unwrap();
                } else {
                    write!(output, "{}", row.get(column).unwrap()).unwrap();
                }
            }

            if i < (self.rows.len() - 1) {
                write!(output, "\n").unwrap();
            } else {
                write!(output, "\nEOF").unwrap();
            }
        }

        output
    }

    pub fn deserialize(input: String) -> Table {
        let mut name = String::new();
        let mut headers = Vec::new();
        let mut rows = Vec::new();

        for (i, line) in input.split("\n").enumerate() {
            match i {
                0 => {name = line.to_string()},
                1 => {headers = line.split("|").map(|s| s.to_string()).collect()},
                _ => {
                    if line == "EOF" {
                        break;
                    }

                    let row_input: Vec<String> = line.split("|").map(|s| s.to_string()).collect();
                    let mut row: HashMap<String, ColumnData> = HashMap::new();
                    for (i, column) in headers.iter().enumerate() {
                        row.insert(
                            column.to_string(),
                            ColumnData::Varchar(row_input[i].clone()),
                        );
                    }
                    rows.push(row);
                }
            }
        }

        Table {
            name,
            headers,
            rows
        }
    }
}

#[derive(Clone)]
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

impl ColumnData {
    pub fn parse(input: &str) -> Result<ColumnData, ()> {
        if let Ok(parsed) = input.parse::<i32>() {
            Ok(ColumnData::Int(parsed))
        } else {
            Ok(ColumnData::Varchar(input.to_string()))
        }
    }
}
