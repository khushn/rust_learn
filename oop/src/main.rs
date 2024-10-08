use blog::Post;
use oop::*;

fn main() {
    let mut ac = AveragedCollection::new();
    ac.add(4);
    ac.add(3);
    ac.add(5);
    println!("ac: {:?}", ac);
    ac.remove(3);
    println!("ac: {:?}", ac);

    // traits
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    //println!("screen: {:?}", screen);
    screen.run();

    // section 17.3
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
