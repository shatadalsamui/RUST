fn main() {
    let ans;
    let str1 = String::from("aauy");

    {
        let str2 = String::from("kjbfcidsbijfnds");
        ans = longest(&str1, &str2);
    }
    // Error: `ans` may reference `str2`, which is dropped here (dangling reference).
    // This demonstrates why explicit lifetimes are needed.
    println!("{}", ans); 
}

fn longest(a: &str, b: &str) -> &str {
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
}
