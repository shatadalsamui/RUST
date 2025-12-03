use std::collections::HashMap;
 
//hashmap - key value pair storage 
fn main() {
    let mut users: HashMap<String, i32> = HashMap::new();

    //inserting element 
    users.insert(String::from("shatadal"), 22);
    users.insert(String::from("shatadal2"), 23);
    users.insert(String::from("shatadal3"), 24);

    println!("{:?}", users);

    //getting an element 
    let user1 = users.get("shatadal");

    println!("{}", user1.unwrap());

    //removing an element using it key 
    users.remove("shatadal2");

    println!("{:?}", users);

    //clearing the whole hashmap 
    users.clear();
    
    println!("{:?}", users);

}
