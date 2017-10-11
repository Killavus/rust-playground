use std::io;

fn fahrenheit_to_celsius(source_degrees: f64) -> f64 {
    (source_degrees - 32.0) / 1.8
}

fn celsius_to_fahrenheit(source_degrees: f64) -> f64 {
    source_degrees * 1.8 + 32.0
}

fn convert_degrees(source_type: &str, degrees: f64) -> f64 {
    if source_type == "C" {
        celsius_to_fahrenheit(degrees)
    } else {
        fahrenheit_to_celsius(degrees)
    }
}

fn main() {
    println!("Temperature Converter");
    println!("Please enter the source type (C - celsius, F - fahrenheit): ");

    let mut source_type = String::new();
    let mut value = String::new();

    io::stdin()
        .read_line(&mut source_type)
        .expect("Failed to read source type");

    if source_type.trim() != "C" && source_type.trim() != "F" {
        println!("Invalid source type.");
        return;
    }

    println!("Now please enter the temperature value: ");

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read temperature value");

    let value: f64 = value.trim().parse().expect("Please enter the number.");

    println!("Converted value is: {}",
             convert_degrees(&source_type, value));
}
