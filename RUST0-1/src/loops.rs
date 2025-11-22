fn main() {
    simple_loops();
    while_loop();
    infinite_loops();
    strings();
}

// simple loops in rust 0..10 the loop will run from 0 to 9 .
fn simple_loops() {
    for i in 0..10 {
        print!("{} ", i);
    }
    println!();
}

//while loop example 
fn while_loop() {
    let mut counter = 0;
    while counter < 5 {
        println!("Counter: {}", counter);
        counter += 1;
    }
}

//breaking in loops on some conditions 
fn infinite_loops(){
    let mut attempts = 0 ; 
    loop {
        attempts += 1 ; 
        println!("Attempt: {}",attempts);

        if attempts >= 3 {
            println!("breaking");
            break;
        }
    }
}

fn strings(){
    let str = String::from("Shatadal Samui");
    println!("First name:{}",get_first_name(str));
}

//loops in strings 
fn get_first_name(str:String) -> String{
    let mut first_name = String::from("");
    for c in str.chars(){
        if c == ' ' {
            break ; 
    }
    first_name.push(c);
}
return first_name;
}