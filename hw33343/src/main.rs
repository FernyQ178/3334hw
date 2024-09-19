// Function to check if the guess is correct, too high, or too low
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0 // Guess is correct
    } else if guess > secret {
        1 // Guess is too high
    } else {
        -1 // Guess is too low
    }
}

fn main() {
    // Secret number (hard-coded)
    let secret_number: i32 = 42;

    // Counter for number of guesses
    let mut guess_count: i32 = 0;

    // Set initial guess bounds
    let mut low: i32 = 1;
    let mut high: i32 = 100;

    // Start guessing loop
    loop {
        // Simulate user input by setting a mutable guess variable (midpoint)
        let guess: i32 = (low + high) / 2;
        guess_count += 1;

        // Call the check_guess function
        let result = check_guess(guess, secret_number);

        // Use if-else to print if the guess was correct, too high, or too low
        if result == 0 {
            println!("Your guess of {} is correct!", guess);
            break; // Exit the loop if the guess is correct
        } else if result == 1 {
            println!("Your guess of {} is too high!", guess);
            high = guess - 1; // Adjust the upper bound
        } else {
            println!("Your guess of {} is too low!", guess);
            low = guess + 1; // Adjust the lower bound
        }
    }

    // After the loop ends, print how many guesses it took
    println!("It took {} guesses to find the correct number.", guess_count);
}
