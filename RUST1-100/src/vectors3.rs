fn main() {
    
    let vec = vec![15, 17, 18, 19, 12, 19, 16];
    even_filter_and_print(&vec); // immutable borrow 
}

fn even_filter_and_print(vec: &Vec<i32>) {

    let mut even_vec = Vec::<i32>::new();

    for &value in vec {
        if value % 2 == 0 {
            even_vec.push(value);
        }
    }

    println!("{:?}", even_vec);
}
