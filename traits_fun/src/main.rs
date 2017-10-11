extern crate traits_fun;

use traits_fun::{Tweet, Summarizable};

fn largest<T: PartialOrd>(list: &[T]) -> &[T] {
    let mut largest_index = 0;

    for (index, item) in list.iter().enumerate() {
        if item > &list[largest_index] {
            largest_index = index;
        }
    }

    &list[largest_index..largest_index + 1]
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let v = vec![1, 2, 3, 4, 5, 6];
    println!("Largest in vec is {}", largest(&v)[0]);

    println!("1 new tweet: {}", tweet.summary());
}