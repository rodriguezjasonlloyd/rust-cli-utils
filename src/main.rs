use std::env::args;

fn main() {
    let mut iter = args().skip(1).peekable();

    if let Some(command) = iter.next() {
        match command.to_lowercase().as_str() {
            "echo" => {
                while let Some(arg) = iter.next() {
                    print!("{arg}");

                    if iter.peek().is_some() {
                        print!(" ");
                    }
                }

                println!();
            }
            _ => println!("Invalid Command Line Utility"),
        }
    }
}
