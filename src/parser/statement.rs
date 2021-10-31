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
}
