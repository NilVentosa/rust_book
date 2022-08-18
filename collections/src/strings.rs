pub fn strings() {
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world.");
    let s3 = s1 + &s2; // Same as fn add(self, s: &str) -> String {}
    println!("{}", s3);
    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    let s1 = String::from("tic");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}
