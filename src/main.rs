mod btree;

fn main() {
    let mut btree = btree::Btree::new(10000);

    let count = 1_000_000;

    let start = std::time::Instant::now();
    for i in 0..count {
        btree.insert(i);
    }
    println!("Inserting {count} elements takes: {:?} seconds", start.elapsed().as_secs() );

    for i in 0..count {
        let start = std::time::Instant::now();
        assert!(btree.search(i), "failed to find {}", i);
        let end = start.elapsed();
        println!("Searching takes: {:?} in nano seconds", end.as_nanos() );
    }
}
