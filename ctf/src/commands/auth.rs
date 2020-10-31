use std::str;

use crate::commands::Token;
use crate::console::Console;

pub struct Auth {
    logged_in: bool,
    user: Option<User>,
}

pub struct User {
    pub username: String,
}

impl User {
    pub fn new(username: &str) -> User {
        User {
            username: username.to_string(),
        }
    }
}

impl Auth {
    pub fn new() -> Self {
        Auth {
            logged_in: false,
            user: None,
        }
    }

    pub fn user(&self) -> &Option<User> {
        &self.user
    }

    pub fn logout(&mut self, console: &mut Console) -> String {
        self.logged_in = false;
        self.user = None;
        console.set_prompt("$".to_string());
        "Logged out!".to_string()
    }

    pub fn login(&mut self, token: &Token<'_>, console: &mut Console) -> String {
        let mut username: Option<&str> = None;
        let mut password: Option<&str> = None;

        for (key, val) in &token.args {
            match key.as_str() {
                "-u" | "--username" => username = Some(&val),
                "-p" | "--password" => password = Some(&val),
                _ => {}
            }
        }

        let read_user;
        let read_pass;
        if username == None || password == None {
            let _ = console.write_str("Username: ");
            read_user = console.read_till_newline(false);

            let _ = console.write_str("Password: ");
            read_pass = console.read_till_newline(true);

            username = Some(&read_user.trim());
            password = Some(&read_pass.trim());
        }

        if username.unwrap() == "zac" && password.unwrap() == "09820" {
            self.logged_in = true;
            self.user = Some(User::new(username.unwrap()));
            console.set_prompt(format!("{}@ctf$", &self.user.as_ref().unwrap().username));
            format!("Logged in as {}!", username.unwrap())
        } else {
            "Incorrect username or password!".to_string()
        }
    }
}
