use std::io;

fn fib(n: u32) -> u32 {
    if n < 2 { 1 } else { fib(n - 1) + fib(n - 2) }
}

fn main() {
    let mut index = String::new();

    println!("Fibonacci number calculator");
    println!("Please enter the index of sequence element you are interested in: ");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read index");

    let index: u32 = index
        .trim()
        .parse()
        .expect("Failed to parse index - be sure it is a number");

    println!("The value of {}-th fibonacci number is {}",
             index,
             fib(index));
}
