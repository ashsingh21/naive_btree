mod btree;

fn main() {
    let mut btree = btree::Btree::new(10);

    let count = 1000_000;

    for i in 0..count {
        let start = std::time::Instant::now();
        btree.insert(i);
        let end = start.elapsed();
        println!("Inserting takes: {:?} in micro seconds", end.as_micros() );
    }

    for i in 0..count {
        let start = std::time::Instant::now();
        assert!(btree.search(i), "failed to find {}", i);
        let end = start.elapsed();
        println!("Searching takes: {:?} in nano seconds", end.as_nanos() );
    }
}
