use std::time::Instant;
use std::fs;

fn main() {
    let start = Instant::now();

    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    
    let elapsed = start.elapsed();
    // debug format:
    //println!("{:?}", elapsed);
    // or format as milliseconds:
    println!("Elapsed: {} ms", 
             (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}