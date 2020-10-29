use std::any::Any;
use serde::Deserialize;
use std::fs;

use crate::commands::{Token, CommandHandler};
use crate::commands::Command;
use crate::Sock;
use std::collections::HashMap;
use std::collections::hash_map::Values;

#[derive(Deserialize)]
pub struct Challenge {
    pub name: String,
    pub description: String
}

pub struct ChallengeCommand {
    pub challenges: Vec<Challenge>
}

impl ChallengeCommand {
    pub fn new() -> Box<ChallengeCommand> {

        let mut challenges: Vec<Challenge> = Vec::new();
        let paths = fs::read_dir("./challenges").expect("Could not find the challenges directory!");
        for path in paths {
            if path.expect("Error while accessing path!").path().ends_with(".toml") {
                let file = fs::read_to_string("challenges/config").expect("Could not find the file!");
                let val: Challenge = toml::from_str(file.as_str()).unwrap();
                challenges.push(val);
            }
        }

        Box::new(ChallengeCommand {
            challenges: challenges
        })
    }

    fn list(&self, token: &Token, _sock: &mut Sock) -> String {
        if token.parameters.len() <= 0 {
            return String::from("Usage: challenge [COMMAND] [OPTION]...\nFor more info, type: challenge -h");
        }

        match token.parameters[0] {
            "list" => {
                let mut output: String = String::from("List of challenges:\n");

                for i in &self.challenges {
                    output.push_str(&i.name.as_str());
                }

                output
            },
            "-h" => {
                String::from("Challenge help:\n\
                \t")
            },
            _ => String::from(format!("Not recognized: '{}'.", token.parameters[0]))
        }
    }
}

impl Command for ChallengeCommand {
    fn handle(&mut self, name: &str, token: &Token, sock: &mut Sock, commands: &Values<String,Box<dyn Command>>) -> Result<String, bool> {
        match name {
            "challenge" => Ok(self.list(token, sock)),
            _ => Err(false)
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}