fn main() {
    let x: i32 = 1;
    //x = 2 ; this line will show error as by deafult variables ion rust are immutable
    println!(" x : {}", x);

    let mut y: i32 = 2;// add the mut keyowrds make it mutable meaning it can be changed or updated ; 
    y = 3;
    println!("y : {}", y)
}
