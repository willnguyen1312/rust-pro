fn _hello_from_thread(i: u32) {
    // This will print the thread id
    println!("Hello from thread {}", i);
}

fn do_something(i: u32) -> u32 {
    let mut result = 1;

    for _ in 0..i {
        result *= 2;
    }

    result
}

fn main() {
    println!("Hello from main thread");

    let mut threads = Vec::new();

    for i in 0..5 {
        threads.push(std::thread::spawn(move || do_something(i)));
    }

    threads.into_iter().for_each(|t| {
        let result = t.join().unwrap();
        println!("Result: {}", result);
    });
}
