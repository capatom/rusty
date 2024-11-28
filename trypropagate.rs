fn main() {
    let numbers = vec![10, 20, 30];
    
    match get_item_from_list(&numbers, 1) {
        Some(item) => println!("Item found: {}", item),
        None => println!("Item not found"),
    }

    match get_item_from_list(&numbers, 5) {
        Some(item) => println!("Item found: {}", item),
        None => println!("Item not found"),
    }
}

// now, ? try operator
fn get_item_from_list(list: &[i32], index: usize) -> Option<i32> {
    // If the index is out of bounds, return None early
    let item = list.get(index)?;
    
    // If the item exists, return it
    Some(*item)
}

// previously, manual boiler plate to handle the Option
fn get_item_from_list(list: &[i32], index: usize) -> Option<i32> {
    // Use `match` to explicitly handle the `Option`
    let item = match list.get(index) {
        Some(value) => *value,  // If Some, dereference the value
        None => return None,    // If None, return early with None
    };

    // Return the found item wrapped in `Some`
    Some(item)
}