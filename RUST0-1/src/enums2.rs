enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64),

}

fn main(){
    let circle1  = Shape::Circle(5.0);
    let sqaure1 = Shape::Square(4.0);
    let rectangle1 = Shape::Rectangle(4.0,5.0);
}