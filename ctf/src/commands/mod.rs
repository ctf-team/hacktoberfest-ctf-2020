extern crate indexmap;
use indexmap::IndexMap;

use crate::commands::auth::Auth;
use crate::Sock;

mod auth;

pub struct Token<'a> {
    pub name: String,
    pub parameters: Vec<&'a str>,
    pub args: IndexMap<String, String>,
}

pub struct CommandHandler {
    auth: Auth,
}

impl CommandHandler {
    pub async fn new() -> Self {
        CommandHandler {
            auth: Auth::new().await,
        }
    }

    pub async fn handle(&mut self, token: &Token<'_>, sock: &mut Sock) -> Option<String> {
        println!(
            "{}\n{}\n{}\n{}",
            format!("Command: {:?}", token.name),
            format!("Args: {:?}", token.parameters),
            format!("Parsed Keys: {:?}", &token.args.keys()),
            format!("Parsed Values: {:?}", &token.args.values())
        );

        match token.name.as_str() {
            "whoami" => match self.auth.user() {
                Some(u) => Some(format!("Logged in as: {}", u.username)),
                None => Some(format!("You are not logged in!")),
            },
            "login" => Some(self.auth.login(token, sock).await),
            "logout" => Some(self.auth.logout().await),
            "exit" => None,
            _ => Some(format!("'{}' not found! Type 'help' to see a list of available commands.", token.name).to_string()),
        }
    }
}
