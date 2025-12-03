
//structs and theri implemention 
struct Rect {
    width: u32,
    height: u32,
}


impl Rect {

    //methods to calc the area and perimeters 
    // &self means the method takes an immutable reference to the instance of Rect
    // This allows the method to access the fields of the struct without taking ownership or modifying them
    fn area(&self) -> u32 {
        return (self.width * self.height);
    }
    fn perimeter(&self) -> u32 {
        return (self.width * 2 + self.height * 2);
    }
}

fn main() {

    //instances of rect
    let rect1 = Rect {
        width: 30,
        height: 50,
    };

    let rect2 = Rect {
        width: 300,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} and the perimeter is {} ",
        rect1.area(),
        rect1.perimeter()
    );
    println!(
        "The area of the rectangle is {} the perimeter is {} ",
        rect2.area(),
        rect2.perimeter()
    );
}
