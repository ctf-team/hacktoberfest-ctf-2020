use tokio::io::Error;
use tokio::prelude::*;

pub struct Sock {
    pub sock: tokio::net::TcpStream,
    pub current_history: i32,
    pub history: Vec<String>,
    pub current_string: String,
    pub prompt: String,
}

impl Sock {
    pub fn new(sock: tokio::net::TcpStream) -> Self {
        Sock {
            sock,
            current_history: -1,
            history: Vec::new(),
            current_string: String::default(),
            prompt: String::from("$"),
        }
    }

    pub async fn read(&mut self, mut buf: &mut [u8]) -> Option<usize> {
        match self.sock.read(&mut buf).await {
            // socket closed
            Ok(n) if n == 0 => None,
            Ok(n) => Some(n),
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                None
            }
        }
    }

    pub async fn read_till_newline(&mut self, mask_password: bool) -> Option<String> {
        let mut output: String = String::default();
        let mut buf: [u8; 512] = [0; 512];
        loop {
            let read = self.read(&mut buf).await;

            if read == None {
                return None;
            }

            let response = self.handle_string(buf, read.unwrap(), &mut output, mask_password).await;
            if response {
                break;
            }
            println!("Output: {}", output);
        }
        if !mask_password && output.len() > 0 {
            self.history.insert(0, output.clone());
        }
        Some(output)
    }

    pub async fn write(&mut self, input: &str) -> Result<(), Error> {
        println!("Writing: {:0x?}", input);
        self.sock.write_all(input.as_bytes()).await
    }

    pub async fn write_buf(&mut self, input: &[u8]) -> Result<(), Error> {
        println!("Writing: {:0x?}", input);
        self.sock.write_all(input).await
    }

    pub async fn handle_string(&mut self, recv: [u8; 512], read: usize, input: &mut String, mask_input: bool) -> bool {
        println!("{:0x?}", &recv[..read]);
        if read > 0 && recv[0] == 0xff {
            let mut iac = false;
            for i in 0..read {
                match recv[i] {
                    b if b == 0xff && !iac => {
                        iac = true;
                    },
                    b if iac => {
                        match b {
                            0xfd => { // do, we reply with wont
                                let code = recv[i+1];
                                match code {
                                    0x34 | 0x01 => {
                                        self.will(code).await;
                                    },
                                    _ => {
                                        self.wont(code).await;
                                    }
                                }
                            },
                            0xfb => { // we reply with dont
                                let code = recv[i+1];
                                match code {
                                    0x01 => {
                                        self.should(code).await;
                                    },
                                    _ => {
                                        self.dont(code).await;
                                    }
                                }
                            },
                            _ => {}
                        }
                        iac = false;
                    },
                    _ => {}
                }
            }
            return false;
        }
        match &recv[..read] {
            [0x7F] | [0x08] => { // backspace
                if input.len() > 0 {
                    let new_string = input.clone()[0..input.len() - 1].to_string();
                    input.clear();
                    input.push_str(new_string.as_str());
                    self.backspace().await;
                } else {
                    self.bell().await;
                }
            },
            [0x03] => { // ctrl+c
                self.current_history = -1;
                input.clear();
                self.prompt_with_pre_input(&mut "^C".to_string()).await;
            },
            [0x0d, 0] | [0x0d, 0x0a] => { // newline
                self.current_history = -1;
                let _ = self.write("\r\n").await;
                return true;
            },
            [0x1b, 0x5b, 0x41] => { // up arrow, go up one in history.
                if self.current_history < (self.history.len() as i32 - 1) {
                    if self.current_history == -1 && self.history.len() == 0 {
                        return false;
                    }
                    self.current_string = input.clone();
                    self.current_history += 1;
                    input.clear();
                    input.push_str(self.history[self.current_history as usize].as_str());
                    self.clear_line().await;
                    self.prompt_with_input_no_newline(input).await;
                }
            },
            [0x1b, 0x5b, 0x42] => { // down arrow, go down one in history
                if self.current_history >= 0 {
                    self.current_history -= 1;
                    if self.current_history > -1 {
                        input.clear();
                        self.current_string = input.clone();
                        input.push_str(self.history[self.current_history as usize].as_str());
                    } else {
                        input.clear();
                        input.push_str(self.current_string.as_str());
                    }
                    self.clear_line().await;
                    self.prompt_with_input_no_newline(input).await;
                }
            },
            [0x1b, 0x5b, 0x43] => { // right arrow, seek right in current input

            },
            [0x1b, 0x5b, 0x44] => { // left arrow, seek left in current input

            },
            i => {
                if (i[0] > 32 && i[0] < 126) || i[0] == 0x20 || i[0] == 0x7e || i[0] == 0x09 {
                    input.push(char::from(i[0]));
                    if mask_input {
                        let _ = self.write("*").await;
                    } else {
                        let _ = self.write_buf(i).await;
                    }
                }
            }
        }

        false
    }

    pub fn set_prompt(&mut self, prompt: String) {
        self.prompt = prompt;
    }

    pub async fn prompt(&mut self) {
        let _ = self.write(format!("\r\n{} ", self.prompt).as_str()).await;
    }

    pub async fn prompt_with_input_no_newline(&mut self, input: &mut String) {
        let _ = self.write(format!("\r{} ", self.prompt).as_str()).await;
        let _ = self.write(input.as_str()).await;
    }

    pub async fn prompt_with_input(&mut self, input: &mut String) {
        self.prompt().await;
        let _ = self.write(input.as_str()).await;
    }

    pub async fn prompt_with_pre_input(&mut self, pre_input: &mut String) {
        let _ = self.write(pre_input.as_str()).await;
        self.prompt().await;
    }

    pub async fn prompt_with_pre_and_post_input(&mut self, pre_input: &mut String, post_input: &mut String) {
        let _ = self.write(pre_input.as_str()).await;
        self.prompt_with_input(post_input).await;
    }

    async fn clear_line(&mut self) {
        let _ = self.write_buf(vec![0xff, 0xf8].as_slice()).await;
        let _ = self.write("                                                                 ").await;
        self.prompt_with_input_no_newline(&mut "".to_string()).await;
    }

    async fn bell(&mut self) {
        let _ = self.write_buf(vec![0x07].as_slice()).await;
    }

    async fn backspace(&mut self) {
        let _ = self.write_buf(vec![0x08, 0x20, 0x08].as_slice()).await;
    }

    async fn dont(&mut self, code: u8) {
        let _ = self.write_buf(vec![0xff, 0xfe, code].as_slice()).await;
    }

    async fn wont(&mut self, code: u8) {
        let _ = self.write_buf(vec![0xff, 0xfc, code].as_slice()).await;
    }

    async fn will(&mut self, code: u8) {
        let _ = self.write_buf(vec![0xff, 0xfb, code].as_slice()).await;
    }

    async fn should(&mut self, code: u8) {
        let _ = self.write_buf(vec![0xff, 0xfd, code].as_slice()).await;
    }
}
