fn main() {
    let mut sum = 0;
    
    for x in 0..1000 {
        if x % 3 == 0 {
            sum += x;
        } else if x % 5 == 0 {
            sum += x;
        }
    }
    
    println!("The sum of all multiples of 3 and 5 below 1000 equals to {}", sum);
}
