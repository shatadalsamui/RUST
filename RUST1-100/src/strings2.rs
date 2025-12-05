fn main() {
    //reaplcing in string 
    let mut name = String::from("Shatadal");
    name.push_str(" Samui");
    println!("{}", name);

    name.replace_range(0..name.len(), "Rivu");
    println!("{}", name);
}
