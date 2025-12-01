struct Rect {  
    width: u32 , 
    height : u32 , 
}

impl Rect { 
    fn area(&self) -> u32 {
        return (self.width * self.height);
    }
}

fn main(){

    let rect1 = Rect {
        width: 30 , 
        height: 50, 
    };

     let rect2 = Rect {
        width: 300 , 
        height: 50, 
    };

    println!("The area of the rectangle is {} ",rect1.area());
    println!("The area of the rectangle is {} ",rect2.area());
}