// Function to check the guess
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        return 0; // Correct guess
    } else if guess > secret {
        return 1; // Guess is too high
    } else {
        return -1; // Guess is too low
    }
}
fn main() {
    // The secret number
    let secret = 42;
    let mut guess: i32;
    let mut attempts = 0;
    loop {
        // Simulating user input
        guess = 33;
        attempts += 1;
        // Check the guess
        match check_guess(guess, secret) {
            0 => {
                println!("Congratulations! You guessed the number {} correctly.", guess);
                break; // Exit the loop if the guess is correct
            }
            1 => println!("Your guess of {} is too high. Try again.", guess),
            -1 => println!("Your guess of {} is too low. Try again.", guess),
            _ => unreachable!(),
        }
    }
    // Print the number of attempts after the loop ends
    println!("It took you {} guesses to find the correct number.", attempts);
}
