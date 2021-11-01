use std::collections::HashMap;
use std::fmt;

pub struct Table {
    pub name: String,
    pub headers: Vec<String>,
    pub rows: Vec<HashMap<String, ColumnData>>,
}

impl Table {
    pub fn new(name: String, headers: Vec<String>) -> Table {
        Table {
            name,
            headers,
            rows: Vec::new(),
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
