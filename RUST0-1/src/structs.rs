struct User {
    active : bool , 
    username : String , 
    email : String , 
    signin_count : u64
}

fn main(){

    // when each user is created all the attributes of that struct 
    // are inside a single stackframe then accordingly strings , vraibles are allocated in heap with a poniter in the stack frame 
    let user1 = User{
        active: true ,
        username : String::from("Shatadalsamui"),
        email : String::from(("someone@example.com")),
        signin_count : 1 ,
    };
 
 println!("{}",user1.username);
}