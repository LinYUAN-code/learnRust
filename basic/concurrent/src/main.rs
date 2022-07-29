use std::{sync::mpsc, thread, time::Duration};

pub fn case1() {
    let thread_name = "eva";
    // move 表示闭包获取ownership
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("{} thread: {}", thread_name, i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    for i in 1..5 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(100));
    }
    handle.join().unwrap();
}

pub fn case2() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let data = String::from("hello from tx");
        println!("send data {}", data);
        tx.send(data).unwrap()
    });
    let data = rx.recv().unwrap();
    println!("recv data {}", data);
}

fn main() {
    // case1();
    case2();
}
