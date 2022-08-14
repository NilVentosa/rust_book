use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");


    // Tupple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("The value of y is {y}");

    // Array
    let _a = [1,2,3,4];
    let _b: [i32; 4] = [1,1,1,1];
    let c = [3;5];
    println!("The fouth element of c is {}", c[3]);


}
