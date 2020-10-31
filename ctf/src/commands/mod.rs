extern crate indexmap;
use indexmap::IndexMap;

use crate::commands::auth::Auth;
use crate::console::Console;

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
    pub fn new() -> Self {
        CommandHandler { auth: Auth::new() }
    }

    pub fn handle(&mut self, token: &Token<'_>, console: &mut Console) -> Option<String> {
        /*console.write(format!(
            "\n{}\n{}\n{}\n{}",
            format!("Command: {:?}", token.name),
            format!("Args: {:?}", token.parameters),
            format!("Parsed Keys: {:?}", &token.args.keys()),
            format!("Parsed Values: {:?}", &token.args.values())
        ));*/

        match token.name.as_str() {
            "whoami" => match self.auth.user() {
                Some(u) => Some(format!("Logged in as: {}", u.username)),
                None => Some(format!("You are not logged in!")),
            },
            "login" => Some(self.auth.login(token, console)),
            "logout" => Some(self.auth.logout(console)),
            "clear" => {
                console.clear();
                Some(String::default())
            }
            "exit" => None,
            _ => Some(
                format!(
                    "'{}' not found! Type 'help' to see a list of available commands.",
                    token.name
                )
                .to_string(),
            ),
        }
    }
}
