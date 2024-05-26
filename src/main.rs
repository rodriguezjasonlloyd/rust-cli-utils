use std::env::args;

fn main() {
    let mut iter = args().skip(1).peekable();

    while let Some(arg) = iter.next() {
        print!("{arg}");

        if iter.peek().is_some() {
            print!(" ");
        }
    }
}
