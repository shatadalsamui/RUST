fn main() {
    let mut vec = Vec::<i32>::new();
    vec.push(15);
    vec.push(17);
    vec.push(18);
    vec.push(19);
    vec.push(12);
    vec.push(19);
    println!("{:?}", vec);

    let new_vec = even_filter(vec);  //ownership transfered to the function

    println!("{:?}", new_vec);
}

fn even_filter(vec: Vec<i32>) -> Vec<i32> {
    
    let mut even_vec = Vec::<i32>::new();

    for value in vec {
        if value % 2 == 0 {
            even_vec.push(value);
        }
    }

    return even_vec;
}
