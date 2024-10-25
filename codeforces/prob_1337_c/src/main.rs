use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct Node {
    val: i32,
    children: Vec<Option<Rc<RefCell<Node>>>>, // Shared ownership of child nodes
}

impl Node {
    // Create a new node
    fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            val,
            children: Vec::new(),
        }))
    }

    // Add a child node using Rc<RefCell<Node>> for shared ownership and mutability
    fn add_child(&mut self, child: Rc<RefCell<Node>>) {
        self.children.push(Some(child)); // Transfer shared ownership to the children vector
    }
}

fn main() {
    // Reading the first line into n and k
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut first_line = input.trim().split_whitespace();

    let n: usize = first_line
        .next()
        .unwrap()
        .parse()
        .expect("Expected a number for n");
    let k: usize = first_line
        .next()
        .unwrap()
        .parse()
        .expect("Expected a number for k");

    println!("n: {}, k: {}", n, k); // Debug print, you can remove this

    // Reading the next n-1 lines into u and v
    let mut root_node: Option<Rc<RefCell<Node>>> = None;
    let mut mymap = HashMap::<i32, Option<Rc<RefCell<Node>>>>::new();
    for _ in 0..n - 1 {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let mut parts = line.trim().split_whitespace();
        let u: usize = parts
            .next()
            .unwrap()
            .parse()
            .expect("Expected a number for u");
        let v: usize = parts
            .next()
            .unwrap()
            .parse()
            .expect("Expected a number for v");

        // You can add code here to process u and v
        println!("u: {}, v: {}", u, v); // Debug print, you can remove this

        // Create child nodes
        let mut child1: Option<Rc<RefCell<Node>>> = None;
        let uu = u as i32;
        if mymap.contains_key(&uu) {
            child1 = mymap[&uu].clone();
        } else {
            child1 = Some(Node::new(u as i32));
            mymap.insert(uu, child1.clone());
        }

        let mut child2: Option<Rc<RefCell<Node>>> = None;
        let vv = v as i32;
        if mymap.contains_key(&vv) {
            child2 = mymap[&vv].clone();
        } else {
            child2 = Some(Node::new(v as i32));
            mymap.insert(vv, child1.clone());
        }

        // Add children to each other (circular references)
        let childw1 = child1.clone().unwrap();
        childw1.borrow_mut().add_child(child2.clone().unwrap());

        let childw2 = child2.clone().unwrap();
        childw2.borrow_mut().add_child(child1.clone().unwrap());

        if u == 1 {
            root_node = child1;
        }

        if v == 1 {
            root_node = child2;
        }
    }

    let mut traversed = HashSet::<i32>::new();
    //println!("root_node: {:?}", root_node);
    dfs(root_node.unwrap(), &mut traversed);
}

fn dfs(n: Rc<RefCell<Node>>, traversed: &mut HashSet<i32>) {
    let binding = n.borrow_mut();
    let nval = binding.val;
    println!("root_node: {:?}, traversed: {:?}", nval, traversed);
    traversed.insert(nval);
    // Iterate over all. children and call dfs on each
    for child in binding.children.iter() {
    	let childunwrap = child.clone().unwrap();
        let childval = childunwrap.borrow_mut().val;
        println!("child node: {:?}", childval);
        if !traversed.contains(&childval) {
            dfs(childunwrap, traversed); // Use Rc::clone to pass the reference
        }
    }
}
