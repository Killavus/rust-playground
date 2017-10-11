#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    fn square(side: u32) -> Rectangle {
        Rectangle {
            length: side,
            width: side,
        }
    }
}

fn main() {
    let rectangle = Rectangle {
        length: 50,
        width: 30,
    };

    let smaller_rectangle = Rectangle {
        length: 40,
        width: 25,
    };

    println!("The area of rectangle is {} square pixels.",
             rectangle.area());

    println!("Can {:?} hold {:?}? {}",
             rectangle,
             smaller_rectangle,
             rectangle.can_hold(&smaller_rectangle));

    let square = Rectangle::square(30);
    println!("Square: {:?}", square);
}