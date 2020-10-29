use tokio::net::TcpListener;
use tokio::prelude::*;

mod commands;
mod parser;
mod sock;

use commands::CommandHandler;
use sock::Sock;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("0.0.0.0:6969").await?;

    println!("Started RusCTF!");

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Client connected at {}.", socket.peer_addr()?);

        let welcome_message = "Welcome to CTF! For a list of commands, please type 'help'.\r\n$ ";
        if let Err(e) = socket.write_all(welcome_message.as_bytes()).await {
            println!("Error while writing welcome message: {:?}", e);
        };

        tokio::spawn(async move {
            let mut sock = Sock::new(socket);

            let mut cmd = CommandHandler::new().await;

            //255, 253, 34,  /* IAC DO LINEMODE */
            // 255, 250, 34, 1, 0, 255, 240 /* IAC SB LINEMODE MODE 0 IAC SE */
            // 255, 251, 1    /* IAC WILL ECHO */
            let iac_will_echo: &[u8] = &vec![0xff, 0xfd, 0x22,
                                             //0xff, 0xfa, 0x22, 0x01, 0x00, 0xff, 0xf0,
                                             0xff, 0xfb, 0x01];
            let _ = sock.write_buf(iac_will_echo).await;

            let iac_line_width: &[u8] = &vec![0xff, 0xfd, 0x08];
            let _ = sock.write_buf(iac_line_width).await;

            // In a loop, read data from the socket and write the data back.
            loop {
                let input = sock.read_till_newline(false).await.unwrap();

                let ret = parser::parse_input(&input);

                match ret {
                    Err(e) => {
                        if e.len() <= 0 {
                            sock.prompt().await;
                        } else {
                            println!("Error: {}", e);
                            sock.prompt_with_input(&mut e.to_string());
                        }
                    }
                    Ok(v) => {
                        // Write the data back
                        let response = cmd.handle(&v, &mut sock).await;
                        match response {
                            Some(v) => {
                                sock.prompt_with_pre_input(&mut v.clone()).await;
                            },
                            None => {
                                sock.prompt().await;
                            }
                        }
                    }
                };
            }
        });
    }
}
