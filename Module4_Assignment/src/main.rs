// Assignment: Interactive File Operations 
// in Rust Using Command::new() and Enums
// Objective:
// Create an interactive Rust program that performs basic file operations 
// (ls, cat, create, remove, pwd) by executing system commands using Command::new(). 
// Use enums to represent different file operations. 
// The program should accept user input via a menu system until the user decides to exit.

use std::io::{self, Write, stdin};
use std::process::Command;

//1. Define the FileOperation Enum
enum FileOperation {
    List(String),               // Directory path
    Display(String),            // File path
    Create(String, String),     // File path and content
    Remove(String),             // File path
    Pwd,                        // Print working directory
}

// User Input Function for help
fn user_input(prompt: &str) -> String { //func. takes prompt mesg. and returns user's input
    let  mut input = String::new(); // creating empty string that'll store user input
    println!("{prompt}"); //prints prompt so user knows
    stdin().read_line(&mut input).expect("Failed to read line"); // reads line into input, crashes if fails
    input.trim().to_string() //removes whitespaces, newline at the end, returns clean string

}


//3. implement the perform_operation Function
fn perform_operation(operation: FileOperation) {
    // Implement command execution based on the operation
    match operation { // pattern match to handle each op. type
        FileOperation::List(directory_path) => { // if op. is List, grab directory_path from enum
             // creating command that runs 'ls'
             let status = Command::new("ls").arg(&directory_path).status().expect("Failed to execute ls");

             if !status.success() { //cehcking to see if 'ls' returned a failed status
                eprintln!("failed to list files in directory: {directory_path}");
             }
        }

        FileOperation::Display(file_path) => { //if op. is Display, grab file_path from enum
            // creating command for 'cat'
            let status = Command::new("cat").arg(&file_path).status().expect("Failed to execute cat");

            if !status.success() { //checking to see if 'cat' returned a failed status
                eprintln!("Failed to display file: {file_path}");
            }
        }

        FileOperation::Create(file_path, content) => { //if op. is Create, grab file_path and content from enum
            let command = format!("echo '{}' > {}", content, file_path); // echo 'content' > file_path
            //creating command for 'sh'
            let output = Command::new("sh").arg("-c").arg(command).status().expect("Failed to create file");

            if output.success() { // if sucessful
                println!("File '{}' created successfully.", file_path);
            } else {
                eprintln!("Failed to create file '{}'.", file_path);
            }
        }

        FileOperation::Remove(file_path) => {
            // creating command for 'rm'
            let status = Command::new("rm").arg(&file_path).status().expect("Failed to remove file");

            if status.success() {
                println!("File '{}' removed successfully.", file_path);
            } else {
                eprintln!("Failed to remove file '{}'.", file_path);
            }
        }

        FileOperation::Pwd => {
            //creating command for 'pwd'
            let status = Command::new("pwd").status().expect("Failed to execute pwd");

            if !status.success() {
                eprintln!("Failed to get working directory.");
            }
        }

    }
}


fn main() {
    println!("Welcome to the File Operations Program!");

    //Implementing User Input Loop with Menu
    loop {
        println!("\nFile Operation Menu:"); // menu header
        println!("1. List file in a directory"); // option 1
        println!("2. List file contents"); // option 2
        println!("3. Create a new file"); //option 3
        println!("4. Remove file"); // option 4
        println!("5. Print working directory"); // option 5
        println!("0. Exit");

        print!("Enter your choice (0-5): "); // user input
        io::stdout().flush().expect("Failed to flush stdout"); // flush so prompt shows before waiting for input

        let choice = user_input(""); //read what user typed
        let op = match choice.as_str() { // matching the user choice as a str
            "1" => { // if 1
                let dir = user_input("Enter file path:"); // asking user
                FileOperation::List(dir)
            }

            "2" => { //if 2
                let path = user_input("Enter file path:"); // asking user
                FileOperation::Display(path)
            }

            "3" => { //if 3
                let path = user_input("Enter file path:"); // asking user
                let content = user_input("Enter content:"); //asking user
                FileOperation::Create(path, content)
            }

            "4" => { // if 4
                let path = user_input("Enter file path:"); // asking user
                FileOperation::Remove(path)
            }

            "5" => FileOperation::Pwd, // if 5, password

            "0" => {
                println!("Goodbye!"); 
                break; //exit loop
            }

            _ => { // if any other input is used
                println!("Invalid menu option. Please enter a number from 0 to 5.");
                continue; //loop back
            }
        };

        perform_operation(op); //run the chosen operation using command::new()
    }
}
