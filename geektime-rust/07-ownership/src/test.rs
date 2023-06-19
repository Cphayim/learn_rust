use std::{
    sync::{Arc, Mutex},
    thread,
};

pub(crate) fn test() {
    let arr = vec![1];
    let s = Arc::new(String::from("Hello"));

    let s1 = s.clone();
    let handler = std::thread::spawn(move || {
        println!("{:?}", arr);
        println!("{}", s1);
    });

    println!("{}", s);
    handler.join().unwrap();
}

pub(crate) fn test2() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for i in 0..10 {
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
