// don't do it this way, use min() on iter :)

macro_rules! find_smallest {
    // Base case: If only one argument is left, return it (this is the smallest by default).
    ($smallest:expr) => {
        $smallest
    };

    // Recursive case: Compare the first argument with the result of the recursive call on the rest of the arguments.
    ($first:expr, $($rest:expr),*) => {{
        let rest_smallest = find_smallest!($($rest),*); // Recursively find the smallest in the rest
        if $first < rest_smallest {
            $first // If the first argument is smaller, return it
        } else {
            rest_smallest // Otherwise, return the smallest from the rest
        }
    }};
}

fn main() {
    // Example usage of the macro
    let smallest = find_smallest!(10, 20, 5, 8, 15);
    println!("The smallest number is: {}", smallest); // 5?
}