use crate::commands::Token;
use crate::sock::Sock;

use std::str;

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
    pub async fn new() -> Self {
        Auth {
            logged_in: false,
            user: None,
        }
    }

    pub fn user(&self) -> &Option<User> {
        &self.user
    }

    pub async fn logout(&mut self) -> String {
        self.logged_in = false;
        self.user = None;
        "Logged out!".to_string()
    }

    pub async fn login(&mut self, token: &Token<'_>, sock: &mut Sock) -> String {
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
            let _ = sock.write("Username: ").await;
            read_user = sock.read_till_newline(false).await.unwrap();

            let _ = sock.write("Password: ").await;
            read_pass = sock.read_till_newline(true).await.unwrap();

            username = Some(&read_user.trim());
            password = Some(&read_pass.trim());
        }

        if username.unwrap() == "zac" && password.unwrap() == "09820" {
            self.logged_in = true;
            self.user = Some(User::new(username.unwrap()));
            sock.set_prompt(format!("{}@ctf$", &self.user.as_ref().unwrap().username));
            format!("Logged in as {}!", username.unwrap())
        } else {
            "Incorrect username or password!".to_string()
        }
    }
}
