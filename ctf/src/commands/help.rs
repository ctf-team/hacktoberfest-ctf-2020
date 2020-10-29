use std::any::Any;

use indoc::indoc;

use crate::commands::{Token, CommandHandler};
use crate::commands::Command;
use crate::Sock;
use std::collections::HashMap;
use std::collections::hash_map::Values;

pub struct HelpCommand {
}

impl HelpCommand {
    pub fn new() -> Box<HelpCommand> {
        Box::new(HelpCommand {})
    }

    fn show_help(&self, _token: &Token, _sock: &mut Sock) -> String {
        let comments = indoc! {"
            Available commands:
            \ttest: Does things
        "};
            
        comments.to_string()
    }
}

impl Command for HelpCommand {
    fn handle(&mut self, name: &str, token: &Token, sock: &mut Sock, commands: &Values<String,Box<dyn Command>>) -> Result<String, bool> {
        match name {
            "help" => Ok(self.show_help(token, sock)),
            _ => Err(false)
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}