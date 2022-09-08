fn main() {
    println!("Hello, world!");
}

fn add_two(num: i32) -> i32 {
    num + 2
}

#[test]
fn add_two_test_one() {
    let expected = 2 + 2;
    assert_eq!(expected, add_two(2));
}
