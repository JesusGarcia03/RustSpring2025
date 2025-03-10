//bool expression that determines if even
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers = [12, 5, 18, 7, 10, 15, 3, 8, 21, 30];

    println!("Analyzing numbers:");
    for &num in numbers.iter() {
        //if num is a multiple of 3 or 5 or both then print string that correlates
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else if is_even(num) {
            println!("{}: Even", num);
        } else {
            println!("{}: Odd", num);
        }
    }

    let mut sum = 0;
    let mut index = 0;
    //will be a while loop that adds up all the values of the array
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of all numbers: {}", sum);
//function that will determine what the largest number is by comparing each of them to eachother
    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}
