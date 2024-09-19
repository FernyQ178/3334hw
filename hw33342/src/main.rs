fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers = [10, 24, 36, 75, 98, 49, 63, 55, 33, 30];

    // Use a for loop to check each number
    for &num in numbers.iter() {
        if is_even(num) {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }

        // FizzBuzz logic
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        }
    }

    // Use a while loop to find the sum of the array
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers: {}", sum);

    // Find the largest number using a loop
    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("The largest number is: {}", largest);
}