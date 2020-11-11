fn main() {
    use std::collections::HashMap;

    let mut random_var = HashMap::new();

    random_var.insert(3, "cat");

    random_var.insert(5, "dog");

    // r is an option - either there is something or there is not
    let r =  random_var.get(&3);
    // will return None using match since no value for &7 exist in hash map
    let c =  random_var.get(&7);

    match r {
        Some(n) => println!("Valid Result: {}", n),
        None => println!("invalid"),
    }

    match c {
        Some(n) => println!("Valid Result: {}", n),
        None => println!("invalid"),
    }
}


//from https://doc.rust-lang.org/std/option/

// Type Option represents an optional value: every Option is either Some and contains a value, or None, and does not. Option types are very common in Rust code, as they have a number of uses:

// Initial values
// Return values for functions that are not defined over their entire input range (partial functions)
// Return value for otherwise reporting simple errors, where None is returned on error
// Optional struct fields
// Struct fields that can be loaned or "taken"
// Optional function arguments
// Nullable pointers
// Swapping things out of difficult situations


// Options are commonly paired with pattern matching to query the presence of a value and take action, always accounting for the None case.

// fn divide(numerator: f64, denominator: f64) -> Option<f64> {
//     if denominator == 0.0 {
//         None
//     } else {
//         Some(numerator / denominator)
//     }
// }