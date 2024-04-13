fn hello_from_thread(i: u32) {
    // This will print the thread id
    println!("Hello from thread {}", i);
}

fn main() {
    println!("Hello from main thread");

    let mut threads = Vec::new();

    for i in 0..5 {
        threads.push(std::thread::spawn(move || hello_from_thread(i)));
    }

    threads.into_iter().for_each(|t| t.join().unwrap());
}
