use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, u32> = HashMap::new();

    map.insert(String::from("pawel"), 422);
    map.insert(String::from("janusz"), 500);
    map.entry(String::from("pawel")).or_insert(466);
    map.entry(String::from("marcin")).or_insert(466);

    println!("{:?}", map);
}
