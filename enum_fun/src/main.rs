fn main() {
    let some_u8_value = Some(0u8);

    if let Some(x) = some_u8_value {
        println!("{}", x);
    }
}
