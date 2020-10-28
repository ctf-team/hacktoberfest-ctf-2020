use std::io;
use rand::Rng;
use std::io::Write;

fn main() {
    let mut rng = rand::thread_rng();
    let num: i32 = rng.gen::<i32>();

    println!("The number I am thinking of cannot be guessed without taking me apart.");
    print!("Guess the number: (hint: {}, '{}') ", num, num.to_string());
    io::stdout().flush().unwrap();
    let (_, line) = read_line();
    println!("Your input: '{}'", line);

    if line == num.to_string() {
        println!("Correct answer!!");
    } else {
        println!("Incorrect answer!");
    }
}

fn read_line() -> (usize,String)  {
    let mut line = String::new();
    let size = std::io::stdin().read_line(&mut line).unwrap();
    let len = line.trim_end_matches(&['\r', '\n'][..]).len();
    line.truncate(len);

    (size,line)
}