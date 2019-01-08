extern crate termios;

use std::process;
use std::io;
use std::io::Read;
use std::io::Write;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use std::str;

fn main() {

    let stdin = 0; 
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone(); 
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];

    loop {
        stdout.lock().flush().unwrap();
        reader.read_exact(&mut buffer).unwrap();

        let s = str::from_utf8(&buffer).unwrap();
        println!("{:?}{}", buffer, s);

        match s {
            "q" => {
                tcsetattr(stdin, TCSANOW, & termios).unwrap(); 
                process::exit(1)
            }
            _ => continue,
        }
    } 

}