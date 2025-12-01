struct User {//same stack frame 
    active : bool , // stack 8 bits 
    username : String , // stack -> heap 
    email : String , //stack -> heap 
    signin_count : u64 // stack 32 bits 
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