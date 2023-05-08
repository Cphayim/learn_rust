use std::{sync::mpsc, thread, time::Duration};

fn main1() {
    // tx 是发送端，rx 是接收端
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi");
        // thread::sleep(Duration::from_millis(10000));
        tx.send(val).unwrap();
    });

    // rx.recv() 会阻塞当前线程线程执行，直到从信道中接收一个值

    loop {
        let received = rx.try_recv();
        if received.is_ok() {
            println!("Got: {}", received.unwrap());
            break;
        } else {
            println!("Not got message");
        }
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
