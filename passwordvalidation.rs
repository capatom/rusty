use std::io::{self, Write};

// Function to check if the password is at least 8 characters long
fn check_length(password: &str) -> bool {
    password.len() >= 8
}

// Function to check if the password contains at least 1 uppercase character
fn check_uppercase(password: &str) -> bool {
    password.chars().any(|c| c.is_uppercase())
}

// Function to check if the password contains at least 1 digit
fn check_digit(password: &str) -> bool {
    password.chars().any(|c| c.is_digit(10))
}

// Function to check if the password contains at least 1 punctuation character
fn check_punctuation(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_punctuation())
}

// Function to check if the password meets all the rules
fn check_all_rules(password: &str) -> bool {
    // List of rule-checking functions
    let rules: Vec<fn(&str) -> bool> = vec![
        check_length,
        check_uppercase,
        check_digit,
        check_punctuation, // You can add more rules as needed
    ];

    // Check all rules and collect their results
    let results: Vec<bool> = rules.iter().map(|rule| rule(password)).collect();

    // If all rules passed (i.e., all are true), return true
    results.iter().all(|&r| r)
}

fn main() {
    // Prompt the user to input a password
    print!("Enter a new password: ");
    io::stdout().flush().unwrap();

    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim(); // Remove any trailing newline

    // Check if the password meets all the rules
    if check_all_rules(password) {
        println!("Password is valid.");
    } else {
        println!("Password is invalid.");
    }
}
