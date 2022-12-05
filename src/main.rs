use std::io::{stdin, Read};

fn main() {
    while let Some(b) = stdin().bytes().next() {
        println!("b: {:?}", b);
    }
}
