fn hello_from_thread() {
    println!("Hello from thread");
}

fn main() {
    println!("Hello from main thread");

    let thread_handler = std::thread::spawn(hello_from_thread);

    thread_handler.join().unwrap();
}
