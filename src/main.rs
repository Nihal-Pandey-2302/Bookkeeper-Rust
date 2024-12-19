use std::io;
use std::fs;
use serde::{Serialize, Deserialize};

fn main() {
    let filename = "library.json";
    let mut books = load_from_file(filename);

    loop {
        println!("\nLibrary Menu:");
        println!("1. Add a book");
        println!("2. Search for a book");
        println!("3. Borrow a book");
        println!("4. Return a book");
        println!("5. Edit a book");
        println!("6. Delete a book");
        println!("7. Show library statistics");
        println!("8. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => {
                let book = get_book_from_user();
                books.push(book);
                save_to_file(&books, filename);
            }
            "2" => {
                println!("Enter your search query (title or author):");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let results = search_books(&books, input.trim());
                if results.is_empty() {
                    println!("No books found.");
                } else {
                    for book in results {
                        book.print_book_info();
                    }
                }
            }
            "3" => {
                println!("Enter the index of the book to borrow:");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(index) = input.trim().parse::<usize>() {
                    if let Some(book) = books.get_mut(index) {
                        book.borrow();
                        save_to_file(&books, filename);
                    } else {
                        println!("Invalid index.");
                    }
                }
            }
            "4" => {
                println!("Enter the index of the book to return:");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(index) = input.trim().parse::<usize>() {
                    if let Some(book) = books.get_mut(index) {
                        book.return_book();
                        save_to_file(&books, filename);
                    } else {
                        println!("Invalid index.");
                    }
                }
            }
            "5" => {
                println!("Enter the index of the book to edit:");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(index) = input.trim().parse::<usize>() {
                    if let Some(book) = books.get_mut(index) {
                        edit_book(book);
                        save_to_file(&books, filename);
                    } else {
                        println!("Invalid index.");
                    }
                }
            }
            "6" => {
                println!("Enter the index of the book to delete:");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(index) = input.trim().parse::<usize>() {
                    delete_book(&mut books, index);
                    save_to_file(&books, filename);
                }
            }
            "7" => show_statistics(&books),
            "8" => break,
            _ => println!("Invalid choice! Please try again."),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
    is_available: bool,
}

impl Book {
    fn print_book_info(&self) {
        println!(
            "Title: {}\nAuthor: {}\nPages: {}\nStatus: {}",
            self.title,
            self.author,
            self.pages,
            if self.is_available { "Available" } else { "Borrowed" }
        );
    }

    fn borrow(&mut self) {
        if self.is_available {
            self.is_available = false;
            println!("You have borrowed \"{}\".", self.title);
        } else {
            println!("\"{}\" is already borrowed.", self.title);
        }
    }

    fn return_book(&mut self) {
        if !self.is_available {
            self.is_available = true;
            println!("You have returned \"{}\".", self.title);
        } else {
            println!("\"{}\" was not borrowed.", self.title);
        }
    }
}

fn get_book_from_user() -> Book {
    let mut input = String::new();
    println!("Enter the title of the book:");
    io::stdin().read_line(&mut input).unwrap();
    let title = input.trim().to_string();

    println!("Enter the author of the book:");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let author = input.trim().to_string();

    println!("Enter the number of pages:");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let pages: u32 = input.trim().parse().unwrap();

    Book {
        title,
        author,
        pages,
        is_available: true,
    }
}

fn edit_book(book: &mut Book) {
    let mut input = String::new();
    println!("Enter a new title (leave blank to keep current):");
    io::stdin().read_line(&mut input).unwrap();
    let title = input.trim();
    if !title.is_empty() {
        book.title = title.to_string();
    }

    println!("Enter a new author (leave blank to keep current):");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let author = input.trim();
    if !author.is_empty() {
        book.author = author.to_string();
    }

    println!("Enter a new page count (leave blank to keep current):");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    if let Ok(pages) = input.trim().parse() {
        book.pages = pages;
    }
}

fn delete_book(books: &mut Vec<Book>, index: usize) {
    if index < books.len() {
        books.remove(index);
        println!("Book removed.");
    } else {
        println!("Invalid index.");
    }
}

fn search_books<'a>(books: &'a Vec<Book>, query: &str) -> Vec<&'a Book> {
    let query_lower = query.to_lowercase();
    books
        .iter()
        .filter(|book| {
            book.title.to_lowercase().contains(&query_lower)
                || book.author.to_lowercase().contains(&query_lower)
        })
        .collect()
}


fn show_statistics(books: &Vec<Book>) {
    let total_books = books.len();
    let borrowed_books = books.iter().filter(|book| !book.is_available).count();

    println!("Total books: {}", total_books);
    println!("Borrowed books: {}", borrowed_books);
    println!("Available books: {}", total_books - borrowed_books);
}

fn save_to_file(books: &Vec<Book>, filename: &str) {
    let data = serde_json::to_string(books).expect("Failed to serialize books");
    fs::write(filename, data).expect("Failed to write file");
}

fn load_from_file(filename: &str) -> Vec<Book> {
    if let Ok(data) = fs::read_to_string(filename) {
        serde_json::from_str(&data).expect("Failed to deserialize books")
    } else {
        Vec::new()
    }
}
