use blog::Post;

fn main() {
    let mut post = Post::new();

    let text = String::from("I ate a salad for lunch today");

    post.add_text(&text);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(&text, post.content());
}
