
fn main() {
    let mut vec = vec![0];
    for x in 0..1000 {
        if x % 3 == 0 {
            vec.push(x);
        } else if x % 5 == 0 {
            vec.push(x);
        }
    }
    let vecsum: u32 = vec.iter().sum();
    println!("The sum of all multiples of 3 and 5 below 1000 equals to {}", vecsum);
}
