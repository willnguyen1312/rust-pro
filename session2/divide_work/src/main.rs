fn main() {
    const N_THREADS: usize = 8;
    let to_add: Vec<u32> = (0..5000).collect();
    let mut threads = Vec::new();
    let chunks = to_add.chunks(N_THREADS);

    for chunk in chunks {
        let my_chunk = chunk.to_owned();
        threads.push(std::thread::spawn(move || my_chunk.iter().sum::<u32>()));
    }

    let mut total = 0;

    for thread in threads {
        total += thread.join().unwrap();
    }

    println!("Total: {}", total);
}
