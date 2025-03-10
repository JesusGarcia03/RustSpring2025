fn clone_and_modify(s: &String) -> String {
    let mut cloned = s.clone(); // Clone the original string
    cloned.push_str("World!");  // Append new word
    cloned // Return the modified clone
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); //prints: "Original: Hello,"
    println!("Modified: {}", modified); //prints: "Modified: Hello, World!"
}
