# **Rust Library Manager**

A simple command-line Library Management System built in Rust. This application enables users to manage a collection of books, track borrowed books, and search for books by title or author. The data is persisted using JSON for easy reusability between sessions.

---

## **Features**
- **Add Books**: Add new books to the library with title, author, and number of pages.
- **Search Books**: Search books by title or author with case-insensitive matching.
- **Borrow and Return**: Track whether books are available or borrowed.
- **Delete Books**: Remove books from the library.
- **View Statistics**: Get insights into the number of books available and borrowed.
- **Data Persistence**: Save library data in a JSON file for reusability.

---

## **Installation**

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/rust-library-manager.git
   cd rust-library-manager
2. **Install dependencies**:
Ensure you have Rust installed. If not, download it from Rust's official website.
3. **Build the project**:
   ```bash
   cargo build
4. **Run the Project**:
   ```bash
   cargo run
## **Usage**

1. **Adding Books**:
   - Enter book details (title, author, and number of pages) when prompted.

2. **Searching Books**:
   - Search by index or title using intuitive commands.

3. **Borrowing and Returning Books**:
   - Borrow or return books with simple commands.

4. **Exiting**:
   - Use the `exit` command to safely quit the program. Your data will be saved automatically.
## **Dependencies**

The project uses the following dependencies:

- **serde**: For serializing and deserializing book data.
- **serde_json**: For reading and writing JSON files.

Add these dependencies in your `Cargo.toml` file:

    ```toml
    [dependencies]
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"

## **Future Enhancements**

The following features are planned for future integration to make the Rust Library Manager more comprehensive:

1. **User Management**:
   - Introduce a `User` system to manage library members.
   - Link borrowed books with individual users.

2. **Inventory Tracking**:
   - Track multiple copies of the same book using fields like `inventory_count` and `borrowed_count`.

3. **Advanced Search**:
   - Add filters for searching books based on availability, author, or multiple criteria.

4. **Sorting and Listing**:
   - Provide options to sort and display all books alphabetically, by author, or by status (borrowed/available).

5. **Generate Reports**:
   - Allow users to generate detailed borrowing trends, popular books, and activity reports.

6. **Graphical User Interface (GUI)**:
   - Build a GUI using Rust libraries like `iced` or `egui`.
   - Alternatively, integrate with a web framework for an online interface.

7. **Admin Controls**:
   - Add admin-only features like bulk uploading books, removing users, and generating system logs.

8. **Cloud Integration**:
   - Store library data in a cloud database for easy access and synchronization across devices.

9. **Authentication**:
   - Implement user authentication to ensure secure access to borrowing and returning functionalities.

## **Contributing**
Contributions are welcome! Feel free to fork the repository and submit a pull request. Please ensure your code adheres to the Rust coding standards and is thoroughly tested.

## **License**
This project is licensed under the MIT License.

## **Author**
Developed by Nihal Pandey. Connect with me to discuss Rust, cybersecurity, and blockchain projects!
