fn main() {
    let mut s = String::from("hello world");

    {
        let word = first_word(&s);
        println!("{}", word);
    }

    let r = &mut s;
    r.push_str("!!");

    println!("{}", r);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s
}