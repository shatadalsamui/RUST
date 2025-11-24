fn main (){
    println!("sum : {}",do_sum(23,43));
}
// a function that takes 2 arguments a and b both as i32 
// and returns a i32 after adding them and note without a return type the function will not work
fn do_sum(a:i32 , b:i32 ) -> i32 {
    return a + b ;
}

