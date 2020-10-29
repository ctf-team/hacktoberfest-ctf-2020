use std::any::Any;

use crate::commands::{Token, CommandHandler};
use crate::Sock;
use crate::commands::Command;
use std::collections::HashMap;
use std::collections::hash_map::Values;

pub struct TestCommand{
}

impl TestCommand {
    pub fn new() -> Box<TestCommand> {
        Box::new(TestCommand {})
    }

    fn test(&self, token: &Token) -> String {
        format!("{}\n{}", format!("This is my test str! Args: {:?}", token.parameters).to_string(), format!("This is my test str! Parsed: {:?}", &token.args).to_string())        
    }
}

impl Command for TestCommand {
    fn handle(&mut self, name: &str, token: &Token, _sock: &mut Sock, commands: &Values<String,Box<dyn Command>>) -> Result<String, bool> {
        match name {
            "test" => Ok(self.test(token)),
            _ => Err(false)
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}