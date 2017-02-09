struct Fibonacci {
    current: u32,
    next: u32,
}

fn fibonacci() -> Fibonacci {
    Fibonacci {
        current: 1,
        next: 1,
    }
}

//TODO implement the iterator for Fibonacci

fn main() {
   // sum up even elements of Fibonacci sequence
   // which not exceed 4.000.000
   let (a, b) = (1, 1);

   println!("The sum of all even Fibonacci elements with value below 4.000.000 equals to: {}", 0);
}
