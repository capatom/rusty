// Tribonacci Iterator struct
struct Tribonacci {
    first: u64,
    second: u64,
    third: u64,
}

// Implementing the Iterator trait for Tribonacci
impl Tribonacci {
    // Constructor to create a new Tribonacci iterator
    fn new() -> Self {
        Tribonacci {
            first: 0,
            second: 1,
            third: 1,
        }
    }
}

// Implementing the Iterator trait for Tribonacci struct
impl Iterator for Tribonacci {
    type Item = u64;

    // The `next` method computes the next Tribonacci number
    fn next(&mut self) -> Option<Self::Item> {
        let next_value = self.first;
        let new_value = self.first + self.second + self.third;
        // Update the sequence to the next set of values
        self.first = self.second;
        self.second = self.third;
        self.third = new_value;
        Some(next_value)
    }
}

fn main() {
    // Create a new Tribonacci sequence iterator (immutable here)
    let tribonacci = Tribonacci::new();

    // Demonstrate using `next()` method
    println!("First 10 Tribonacci numbers using next():");
    for num in tribonacci.take(10) {
        println!("{}", num);
    }

    // Create a new Tribonacci iterator for further operations
    let tribonacci = Tribonacci::new();

    // Demonstrate using `skip()` and `take()`
    println!("\nTribonacci numbers skipping the first 5 and taking the next 5:");
    let skipped_tribonacci = tribonacci.skip(5).take(5);
    for num in skipped_tribonacci {
        println!("{}", num);
    }

    // Try skipping some numbers and demonstrating iterator behaviors
    let tribonacci = Tribonacci::new();
    println!("\nTribonacci numbers after skipping 3 and taking 7:");
    for num in tribonacci.skip(3).take(7) {
        println!("{}", num);
    }
}
