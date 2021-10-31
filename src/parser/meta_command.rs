pub enum MetaCommand {
    Exit
}

impl MetaCommand {
    pub fn parse(input: &str) -> Result<MetaCommand, ()> {
        match input {
            ".exit" => Ok(MetaCommand::Exit),
            _ => Err(())
        }
    }
}
