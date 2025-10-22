use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // Try to create a file for writing
    let mut file = File::create(filename).expect("Unable to create file");

    // Write each book as "title,author,year"
    for book in books {
        writeln!(file, "{},{},{}", book.title, book.author, book.year)
            .expect("Unable to write to file");
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut books = Vec::new();

    // Read each line and split by commas
    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.trim().split(',').collect();
            if parts.len() == 3 {
                let title = parts[0].to_string();
                let author = parts[1].to_string();
                // Parse year safely
                if let Ok(year) = parts[2].parse::<u16>() {
                    books.push(Book { title, author, year });
                }
            }
        }
    }
    books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
        Book { title: "The Catcher in the Rye".to_string(), author: "J.D. Salinger".to_string(), year: 1951 },
        Book { title: "Harry Potter".to_string(), author: "J.k. Rowling".to_string(), year: 1990 },
        Book { title: "The Hunger Games".to_string(), author: "Suzanne Collins".to_string(), year: 2008 },
        Book { title: "The Lord of the Rings".to_string(), author: "J.R.R Tolkien".to_string(), year: 1937 },
        
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}
