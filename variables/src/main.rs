fn main() {
    let x = 5;
    println!("The value of x is {x}");

    {
        let x = "hello";
        println!("The value of x in the block is {x}");
    }

    println!("The value of x is {x}");

    let guess:u32 = "42".parse().expect("Not a numer");
    println!("{guess}");
}
