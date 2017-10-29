extern crate forth;

use std::io;
use std::io::prelude::*;
use forth::Forth;

fn main() {
    let mut forth = Forth::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush output");

        let mut line = String::new();
        io::stdin().read_line(&mut line).expect(
            "Failed to read from input",
        );

        if let Err(e) = forth.eval(&line) {
            println!("Error: {:?}", e);
        }

        println!("{:?}", forth.stack());
    }
}
