fn concat_strings(word1: &String, word2: &String) -> String {
    format!("{}{}", word1, word2)
}

fn main() {
    let word1 = String::from("Hello, ");
    let word2 = String::from("World!");
    let result = concat_strings(&word1, &word2);
    println!("{}", result); // Should print: "Hello, World!"
}
