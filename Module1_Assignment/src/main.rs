// Did all 3 Assignments on here

// // Assignment 1: Temperature Converter

// // Declare a constant for the freezing point of water in Fahrenheit (32°F).
// const freeze_pointF: f64 = 32.0;

// // Implement two functions:
// // fahrenheit_to_celsius(f: f64) -> f64: Converts Fahrenheit to Celsius
// fn fahrenheit_to_celsius(f: f64) -> f64 {
//     (f - freeze_pointF) * 5.0 / 9.0
// }

// // celsius_to_fahrenheit(c: f64) -> f64: Converts Celsius to Fahrenheit
// fn celsius_to_fahrenheit(c: f64) -> f64 {
//     c * 9.0 / 5.0 + freeze_pointF
// }


// fn main() {
//     // Declare a mutable variable with a temperature in Fahrenheit
//     let mut temperature_F: f64 = 32.0;

//     // Convert it to Celsius using your function and print the result
//     let temperature_C = fahrenheit_to_celsius(temperature_F);
//     println!("{temperature_F} F is equal to {temperature_C:.2} C");

//     // Use a loop to convert and print the next 5 integer temperatures 
//     // (e.g., if you start with 32°F, print conversions for 33°F, 34°F, 35°F, 36°F, and 37°F)
//     for _ in 1..=5 {
//         temperature_F += 1.0;
//         let converted = fahrenheit_to_celsius(temperature_F);
//         println!("{temperature_F} F is equal to {converted:.2} C");
//     }
// }

// ////////////////////////////////

// // Assignment 2: Number Analyzer

// // Create a Rust program that analyzes a series of numbers. The program should:
// // Create an array of 10 integer numbers of your choice.
// // Implement a function is_even(n: i32) -> bool that returns true if a number is even, false otherwise.
// fn is_even(n: i32) -> bool {
//     n % 2 == 0
// }

// fn main() {
// // Use a for loop to iterate through the array and for each number:
// // Print whether it's even or odd using your is_even function
// // If the number is divisible by 3, print "Fizz" instead
// // If the number is divisible by 5, print "Buzz" instead
// // If it's divisible by both 3 and 5, print "FizzBuzz"
//     let numbers: [i32; 10] = [3, 8, 10, 33, 45, 30, 5, 12, 23, 21]; // creating the array of 10 numbers
//     for number in numbers {
//         if number % 3 == 0 && number % 5 == 0 {
//             println!("{number}: FizzBuzz");
//         } else if number % 3 == 0 {
//             println!("{number}: Fizz");
//         } else if number % 5 == 0 {
//             println!("{number}: Buzz");
//         } else {
//             if is_even(number) {
//                 println!("{number}: Even");
//             } else {
//                 println!("{number}: Odd");
//             }
//         }
//     }

// // Use a while loop to find and print the sum of all numbers in the array.
//     let mut index = 0;
//     let mut sum = 0;

//     while index < numbers.len() {
//         sum += numbers[index];
//         index += 1;
//     }

//     println!("Sum of all numbers: {sum}");

// // Use a loop to find and print the largest number in the array.
// let mut largest = numbers[0];

//     for number in numbers {
//         if number > largest {
//             largest = number;
//         }
//     }
//     println!("Largest number: {largest}");
// }

////////////////////////////////////////////

// Assignment 3: Guessing Game

// Create a simple number guessing game in Rust. The program should:
// Use a mutable variable to store a "secret" number (you can hard-code this).
// Implement a function check_guess(guess: i32, secret: i32) -> i32 that returns:
// 0 if the guess is correct
// 1 if the guess is too high
// -1 if the guess is too low
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
// In the main function:
// Use a loop to repeatedly:
// Set a mutable guess variable to a number of your choice (simulating user input)
    let mut secret_number: i32 = 21;

    let mut guess: i32 = 0;
    let mut guess_count: i32 = 0;

// Call the check_guess function
// Use an if-else expression to print whether the guess was correct, too high, or too low
// If the guess was correct, break the loop
    loop {

        guess_count += 1;

        if guess_count == 1 {
            guess = 3;
        } else if guess_count == 2 {
            guess = 15;
        } else {
            guess = 21;
        }
    
        let result = check_guess(guess, secret_number);

        if result == 0 {
            println!("Guess {guess}: Correct!");
            break;
        } else if result == 1 {
            println!("Guess {guess}: Too High!");
        } else {
            println!("Guess {guess}: Too Low!");
        }
    }

// After the loop ends, print how many guesses it took (you'll need to track this in a variable)
// These assignments strictly use only the concepts covered in the provided materials: 
// variables, mutability, basic data types (integers, booleans), arrays, functions, if-else expressions, and the three types of loops (loop, while, for).
    println!("It took {guess_count} guesses to find the secret number.");
}