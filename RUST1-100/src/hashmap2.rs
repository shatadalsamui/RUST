use std::collections::HashMap;

fn main() {
    let mut hm: HashMap<String, i32> = HashMap::new();

    hm.insert(String::from("shatadal"), 22);
    hm.insert(String::from("shatadal2"), 23);
    hm.insert(String::from("shatadal3"), 24);

    println!("{:?}", hm);

    let input_vec = vec![
        (String::from("SHATADAL4"), 25),
        (String::from("shatadal6"), 32),
    ];

    for (key, value) in input_vec {
        hm.insert(key, value);
    }

    println!("{:?}", hm);
}
