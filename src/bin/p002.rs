struct Fibonacci {
    current: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let temp = self.current +self.next;
        self.current = self.next;
        self.next = temp;
        Some(self.current)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci {
        current: 1,
        next: 1,
    }
}

fn main() {
    //TODO print sum
    //let sum = fibonacci()
    //    .filter(|f| f % 2 == 0) // even elements
    //    .take_while(|n| n <= &bla) // below 4 mil
    //    .sum();
    println!("The sum of all even Fibonacci elements with value below 4.000.000 equals to: {}", sum);
}
