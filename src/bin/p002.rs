struct Fibonacci {
    current: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let temp = self.current + self.next;
        self.current = self.next;
        self.next = temp;

        // Some is always returned
        Some(self.current)
    }
}

// sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci {
        current: 1,
        next: 1,
    }
}

fn main() {
    let mut sum:u32 = 0;
    for i in fibonacci().filter(|&x| x % 2 == 0).take_while(|&x| x < 4_000_000) {
        sum += i
    }

    println!("The sum of all even Fibonacci elements with value below 4.000.000 equals to: {}", sum)
}
