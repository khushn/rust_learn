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
}
