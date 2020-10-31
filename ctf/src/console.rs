extern crate pancurses;
use pancurses::*;

use crate::commands::CommandHandler;
use crate::parser;
use std::thread::sleep;
use std::time::Duration;

pub struct Console {
    window: Window,
    prompt: String,
    current_history: i32,
    history: Vec<String>,
    current_string: String,
}

impl Console {
    pub fn run(&mut self) {
        self.write("Welcome to CTF! For a list of commands, please type 'help'.".to_string());
        self.prompt(true);

        let mut cmd = CommandHandler::new();
        loop {
            self.window.refresh();
            let y = self.window.get_max_y();
            if y - 1 == self.window.get_cur_y() {
                self.window.setscrreg(0, y - 1);
                self.window.scrollok(true);
            }

            let input = self.read_till_newline(false);
            let ret = parser::parse_input(&input);

            match ret {
                Err(e) => {
                    if e.len() <= 0 {
                        self.prompt(false);
                    } else {
                        self.write(format!("Error: {}", e));
                    }
                }
                Ok(v) => {
                    let response = cmd.handle(&v, self);
                    match response {
                        Some(v) => {
                            self.write(format!("{}", v.clone()));
                            self.prompt(true);
                        }
                        None => break,
                    }
                }
            };
        }

        Console::destroy();
    }

    pub fn new() -> Console {
        let window = initscr();
        window.refresh();
        window.keypad(true);
        window.nodelay(true);
        raw();
        noecho();

        Console {
            window,
            prompt: "$".to_string(),
            current_history: -1,
            history: Vec::new(),
            current_string: String::default(),
        }
    }

    pub fn destroy() {
        endwin();
    }

    pub fn write(&self, input: String) {
        self.window.addstr(input);
    }

    pub fn write_str(&self, input: &str) {
        self.window.addstr(input.to_string());
    }

    pub fn write_chrs(&self, input: &[char]) {
        for i in input {
            self.window.addch(*i);
        }
    }

    pub fn set_prompt(&mut self, prompt: String) {
        self.prompt = prompt;
    }

    pub fn clear(&mut self) {
        self.window.clear();
    }

    pub fn prompt(&self, pre: bool) {
        if pre {
            self.window.addstr("\n");
        }
        self.window.addstr(format!("{} ", &self.prompt));
    }

    pub fn read_till_newline(&mut self, mask: bool) -> String {
        let mut input: String = String::default();
        let mut line: u32 = 0;
        loop {
            let keypress = self.window.getch();
            match keypress {
                None => (sleep(Duration::new(0, 1000))),
                c if self.matches_any(c, &[Input::Character('\u{8}'), Input::KeyBackspace]) => {
                    let offset = (self.window.get_cur_x() as u32 * (line + 1)) as usize;
                    let calculated_offset = (offset - self.prompt.len() - 1);
                    if input.len() > 0 && calculated_offset > 0 {
                        if input.len() == calculated_offset {
                            input.remove(input.len() - 1);
                            self.write_chrs(&['\u{8}', '\u{20}', '\u{8}']);
                        } else {
                            input.remove(calculated_offset - 1);
                            self.clear_line();
                            self.prompt(false);
                            self.write(input.to_string());
                            self.window.mv(self.window.get_cur_y(), (offset - 1) as i32);
                        }
                    } else {
                        beep();
                    }
                }
                Some(Input::KeyUp) => {
                    if self.current_history < (self.history.len() as i32 - 1) {
                        if self.current_history == -1 && self.history.len() == 0 {
                            continue;
                        }
                        if self.current_history == -1 {
                            self.current_string = input.clone();
                        }
                        self.current_history += 1;
                        input.clear();
                        input.push_str(self.history[self.current_history as usize].as_str());
                        self.clear_line();
                        self.prompt(false);
                        self.write(input.clone());
                    }
                }
                Some(Input::KeyDown) => {
                    if self.current_history >= 0 {
                        self.current_history -= 1;
                        if self.current_history > -1 {
                            input.clear();
                            input.push_str(self.history[self.current_history as usize].as_str());
                        } else {
                            input.clear();
                            input.push_str(self.current_string.as_str());
                        }
                        self.clear_line();
                        self.prompt(false);
                        self.write(input.clone());
                    }
                }
                Some(Input::KeyLeft) => {
                    let min_x = (self.window.get_beg_x() + self.prompt.len() as i32 + 1) as i32;
                    let (mut curr_y, curr_x) = self.window.get_cur_yx();
                    if line > 0 {
                        line -= 1;
                        curr_y -= 1;
                    }
                    if curr_x > min_x {
                        self.window.mv(curr_y, curr_x - 1);
                    }
                }
                Some(Input::KeyRight) => {
                    // todo: only allow maximum right of string len
                    let (min_y, min_x) = self.window.get_beg_yx();
                    let (max_y, max_x) = self.window.get_max_yx();
                    let (mut curr_y, mut curr_x) = self.window.get_cur_yx();
                    if curr_x < max_x {
                        self.window.mv(curr_y, curr_x + 1);
                    } else {
                        line += 1;
                        curr_y += 1;
                        curr_x = max_x;
                    }
                }
                Some(Input::Character(c)) => {
                    let offset = self.window.get_cur_x() as usize;
                    let calculated_offset =
                        (offset - (if line > 0 { 0 } else { self.prompt.len() + 1 }));
                    if self.handle_char(&mut input, offset, calculated_offset, c, mask) {
                        self.current_history = -1;
                        break;
                    }
                    if input.len() > (self.window.get_max_x() as u32 * (line + 1)) as usize {
                        print!("next line!");
                        line += 1;
                    }
                }
                Some(input) => {
                    self.window.addstr(&format!("{:?}", input));
                }
            }
        }

        if !mask && input.len() > 0 {
            self.history.insert(0, input.clone());
        }

        input
    }

    fn handle_char(
        &mut self,
        input: &mut String,
        offset: usize,
        calculated_offset: usize,
        c: char,
        mask: bool,
    ) -> bool {
        match c {
            '\n' | '\r' => {
                self.window.mv(
                    self.window.get_cur_y(),
                    (input.len() + self.prompt.len() + 1) as i32,
                );
                self.write_str("\n");
                return true;
            }
            '\u{3}' => {
                self.current_history = -1;
                input.clear();
                self.write_str("^C\n");
                self.prompt(false);
            }
            _ => {
                if input.len() == calculated_offset {
                    input.push(c);
                } else {
                    input.insert(calculated_offset, c);
                    self.clear_line();
                    self.prompt(false);
                    self.write(input.to_string());
                    self.window.mv(self.window.get_cur_y(), (offset + 1) as i32);
                    return false;
                }

                if mask {
                    self.window.addch('*');
                } else {
                    // just repaint entire line?
                    self.window.addch(c);
                }
            }
        };

        false
    }

    fn matches_any(&self, input: Option<Input>, matches: &[Input]) -> bool {
        let unwrapped = input.unwrap();
        for item in matches {
            if unwrapped == *item {
                return true;
            }
        }

        false
    }

    fn clear_line(&self) {
        self.window.mv(self.window.get_cur_y(), 0);
        self.window.clrtoeol();
    }
}
