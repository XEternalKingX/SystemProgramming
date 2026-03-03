// Rust Assignment: Book Catalog
// Objective:
// Create a simple Book Catalog system in Rust 
// that demonstrates struct usage and file I/O operations.

// starter code from assignment
use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    let mut file = File::create(filename) //create file 
        .expect("Failed to create file"); //unwraps

    //goes through each book in vector
    for book in books {
        //book data in file, writeln! adds newline at the end
        writeln!(file, "{},{},{}", book.title, book.author, book.year)
            .expect("Failed to write to file"); //unwraps
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let file = File::open(filename) //open file
        .expect("Failed to open file"); //unwraps

    let reader = BufReader::new(file); // wrap the file in BufReader

    let mut books: Vec<Book> = Vec::new(); //creating an empty vector to store books

    //each line in file
    for line in reader.lines() {
        let line = line.expect("Failed to read line"); //unwraps
        let parts: Vec<&str> = line.split(',').collect(); // split line using comma
        
        // if lines have 3 parts: title, author, year
        if parts.len() == 3 {
            let title = parts[0].to_string();
            let author = parts[1].to_string();
            let year: u16 = parts[2]
                .parse()
                .expect("Year must be a number"); //unwraps

            // creating Book struct and adding it into vector 
            books.push(Book { title, author, year });
        }
    }
    // return the vector of loaded books
    books
    
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}