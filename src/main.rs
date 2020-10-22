//this line is used to get nice output of the rectangle 
//since it is not a basic type, rust doesnt know how to handle it
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 15,
        height: 25,
    };

    let rect3 = Rectangle {
        width: 100,
        height: 10,
    };

    let sq = Rectangle::square(5);

    println!("The area of the rectangleis : {}",);
    println!("rect1 is: {:#?}", rect1);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("sq is: {:#?}", sq);
    println!("The area of sq is: {}",sq.area());

}
//the use of impl Rectangle allows the use of self in the methods
impl Rectangle{
    //method to calculate the area of a rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // a function to check whether one rectangle can hold another
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
    //here we create an associated function called square for making square shaped rectangles
    fn square(size: u32)->Rectangle{
        Rectangle{
            width:size,
            height:size,
        }
    }
}
