#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
    
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        rect.width < self.width && rect.height < self.height
    }
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2: Rectangle = Rectangle {
        width: 20,
        height: 0,
    };


    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "rect2 can fit inside rect1: {}",
        rect1.can_hold(&rect2)
    );

    // println!("{:#?}", rect1);
}

// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }
