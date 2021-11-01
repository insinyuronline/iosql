pub enum MetaCommand {
    Exit,
}

impl MetaCommand {
    pub fn parse(input: &str) -> Result<MetaCommand, ()> {
        match input.to_lowercase().split_whitespace().next() {
            Some(".exit") => Ok(MetaCommand::Exit),
            _ => Err(()),
        }
    }
}
