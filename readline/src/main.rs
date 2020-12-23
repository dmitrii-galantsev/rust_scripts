use std::io::{self};

fn main() {
    let welcome_string = "Welcome to readline! Input some string";
    println!("{}", welcome_string);

    let mut in_string = String::new();
    match io::stdin().read_line(&mut in_string) {
        Ok(line) => {
            println!("{} bytes read", line);
            println!("{}", in_string);
        }
        Err(error) => println!("error: {}", error),
    }

    println!("Goodbye!");
}
