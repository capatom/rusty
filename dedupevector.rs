use std::collections::HashSet;

fn main() {
    // Step 1: Create a vector with 10 integers, including duplicates
    let mut numbers = vec![1, 3, 5, 7, 9, 1, 2, 3, 4, 5];
    
    // Print the original vector
    println!("Original vector: {:?}", numbers);

    // Step 2: Deduplicate by sorting and then using `dedup()`
    numbers.sort();  // Sort the vector first
    numbers.dedup(); // Remove consecutive duplicates
    println!("Deduplicated (sorted then deduped) vector: {:?}", numbers);

    // Step 3: Create a new vector and deduplicate by converting to a HashSet and back
    let numbers_with_duplicates = vec![1, 3, 5, 7, 9, 1, 2, 3, 4, 5];
    let unique_numbers: HashSet<i32> = numbers_with_duplicates.into_iter().collect();
    let deduplicated_vec: Vec<i32> = unique_numbers.into_iter().collect();
    
    println!("Deduplicated vector using HashSet: {:?}", deduplicated_vec);
}
