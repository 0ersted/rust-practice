use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

fn main() {
    // 16.1 basic thread
    println!("\nthreading!\n");
    let handle = thread::spawn(||{
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // 16.2 message passing
    println!("\nchannel!\n");
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // 16.3 mutex
    println!("\nmutex!\n");
    let counter = Arc::new(Mutex::new(0)); // Mutex<T> provides intirior mutablility
    let mut handles = vec![];

    for _ in 0..10 {
        let _counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = _counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
