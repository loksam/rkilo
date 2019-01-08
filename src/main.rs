use std::io::{Write, stdout, stdin};
use std::process;

fn main() {

    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        s.pop();

        match &*s {
            "q" => process::exit(1),
            _ => continue,
        }

    }

}
