fn sum(total: &mut i32, low: i32, high: i32) {
    *total = (low..=high).sum(); // Calculate sum and store it in `total`
}

fn main() {
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("Total sum: {}", total); //prints: "Total sum: 5050"
}
