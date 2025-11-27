fn main(){
    let x = 1; // created in main's stack frame
    {
        let y = 2; // created in a new scope inside main's stack frame
    }

   // println!("{}",y); error: y is not accessible here because its scope has ended
}