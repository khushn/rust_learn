use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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
            thread::sleep(Duration::from_millis(100));
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
            thread::sleep(Duration::from_millis(100));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    println!("\n-------- Mutexes and shared state concurrency -------");
    //mutex
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num += 1;
    }

    println!("m = {m:?}");

    // multiple threds using a mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
    	let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
