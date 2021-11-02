pub enum MetaCommand {
    Exit,
    Print,
}

impl MetaCommand {
    pub fn parse(input: &str) -> Result<MetaCommand, ()> {
        match input.to_lowercase().split_whitespace().next() {
            Some(".exit") => Ok(MetaCommand::Exit),
            Some(".print") => Ok(MetaCommand::Print),
            _ => Err(()),
        }
    }
}
