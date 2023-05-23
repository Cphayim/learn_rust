use std::{sync::Mutex, thread};

fn main1() {
    let m = Mutex::new(5);

    {
        // 使用 lock 方法获取锁，以访问互斥器中的数据
        // 这个调用会阻塞当前线程，直到我们拥有锁为止
        let mut num = m.lock().unwrap();
        *num = 6;
        // 这里由于 lock 被 num 获取了，且未释放（当前作用域未结束）
        // 因此 num2 无法获取锁，但 lock 方法阻塞了线程，程序将无法继续执行
        // let mut num2 = m.lock().unwrap();
        // *num2 = 7;
    }

    println!("m = {:?}", m);
}

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
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
