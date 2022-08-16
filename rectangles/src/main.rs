
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }

    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

}

fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 50,
    };

    let rect2 = Rectangle {
        height: 10,
        width: 40,
    };

    let rect3 = Rectangle {
        height: 60,
        width: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("A square: {:?}", Rectangle::square(4));

    let rect4 = Rectangle::new(20,35);

    println!("A rectangle: {:?} and its area {}", rect4, rect4.area());
}

