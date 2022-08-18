use std::collections::HashMap;

fn main() {
    assert_eq!((10, 10), median_mode(vec![1, 10, 3, 45, 1, 10, 10, 45]));
    assert_eq!(
        String::from("irst-fay apple-hay"),
        pig_latin(String::from("first apple"))
    );
}

// Convert strings to pig latin. The first consonant of each word is moved to the end
// of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with
// a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!
fn pig_latin(text: String) -> String {
    let mut result: String = String::new();
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    for word in text.split_whitespace() {
        if vowels.contains(&word.chars().next().unwrap()) {
            result.push_str(word);
            result.push_str(&String::from("-hay "));
        } else {
            result.push_str(&word[1..]);
            result.push_str(&String::from("-"));
            result.push_str(&word.chars().next().unwrap().to_string());
            result.push_str(&String::from("ay "));
        }
    }
    result.pop();
    result
}

// Given a list of integers, use a vector and return the median
// (when sorted, the value in the middle position) and mode
// (the value that occurs most often; a hash map will be helpful here) of the list.
fn median_mode(numbers: Vec<i32>) -> (i32, i32) {
    let mut cloned = numbers.clone();
    cloned.sort();
    let median = cloned[cloned.len() / 2];

    let mut times = HashMap::new();
    let mut mode_count: i32 = 0;
    let mut mode: i32 = 0;

    for i in numbers {
        let count = times.entry(i).or_insert(0);
        *count += 1;
        if count > &mut mode_count {
            mode_count = *count;
            mode = i;
        }
    }
    (median, mode)
}
