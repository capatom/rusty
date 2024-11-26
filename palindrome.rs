fn main() {
    let input = "A man, a plan, a canal, Panama!";
    if is_palindrome(input) {
        println!("'{input}' is a palindrome.");
    } else {
        println!("'{input}' is not a palindrome.");
    }
}

// Core function to check if a string is a palindrome
fn is_palindrome(s: &str) -> bool {
    // Clean the string: keep only alphanumeric characters and convert to lowercase
    let cleaned: String = s.chars()
        .filter(|c| c.is_alphanumeric())  // Keep only alphanumeric characters
        .map(|c| c.to_lowercase())        // Convert to lowercase
        .flatten()                        // Flatten the iterator to get a sequence of characters
        .collect();                       // Collect into a String (can infer from declaration)

    // Compare the cleaned string with its reverse
    cleaned == cleaned.chars().rev().collect::<String>()    // Again, but can't infer String so needs to be explicit for equivalence to work
}
