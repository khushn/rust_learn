use std::thread;
use std::time::Duration;
use std::sync::mpsc;


fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();


    let v = vec![1, 2, 3];

    // move keyword is needed to pass the ownership of v to the thread
    println!("\n-------- move keyword for threads -------");
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();

    // channels (borrowed from golang)
    println!("\n-------- Channels (similar to golang) -------");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // need to wait, because we are using try_recv() instead of recv()
    // and it will return error if channel empty
    thread::sleep(Duration::from_millis(1));

    let gottenval = rx.try_recv().unwrap();
    println!("gottenval: {:?}", gottenval);


    // sending multiple values in a channel
    println!("\n-------- sending multiple values in a channel -------");
    let (tx, rx) = mpsc::channel();

    // we clone the transmitter for use in another child thread
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // multiple sender case
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

}