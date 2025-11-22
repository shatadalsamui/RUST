fn main(){
    conditionals();
}

pub fn conditionals() {
    let x = 98;

    let is_even = is_even(x);

    if is_even {
        println!("{} is even", x);
    } else {
        println!("{} is odd ", x);
    }
}

fn is_even(x: i32) -> bool {
    return x % 2 == 0;
}
