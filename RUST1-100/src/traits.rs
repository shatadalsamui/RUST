// Defines a trait as a blueprint for summarization.
pub trait Summary { 
    fn summarize(&self) -> String;
}

// Defines a trait as a blueprint for summarization.
struct User { 
    name: String,
    age: u32,
}

// Implements the Summary trait for the User struct.
impl Summary for User { 
    fn summarize(&self) -> String {
        return format!("User {} is {} years old ", self.name, self.age);
    }
}

fn main() {
    let user = User {
        name: String::from("Shatadal"),
        age: 22,
    };

    println!("{}", user.summarize());
}
