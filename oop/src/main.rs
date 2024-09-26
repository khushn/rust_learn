
use oop::*;

fn main() {
    let mut ac = AveragedCollection::new();
    ac.add(4);
    ac.add(3);
    ac.add(5);
    println!("ac: {:?}", ac);
    ac.remove(3);
    println!("ac: {:?}", ac);
}
