use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input: u32 = input.trim().parse().unwrap();
    let foo: u32 = input - 100;

    println!("{}", foo);
}