use std::collections::HashMap;

pub fn hashmaps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Green"), 30);
    scores.entry(String::from("Purple")).or_insert(20);
    scores.entry(String::from("Blue")).or_insert(2000);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{}", score.unwrap());
    println!("{}", scores.get(&"Red".to_string()).unwrap_or(&0));

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
