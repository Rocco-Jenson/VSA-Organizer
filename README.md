# VSA Organizer

VSA Organizer is a **Tauri-based desktop application** designed to help users manage a list of completed classes. It allows users to input, read, search, and remove class records efficiently.

## Features

- **Input a New Completed Class** – Add class records with student name, date, and cost.
- **Read All Completed Classes** – View the full list of recorded classes.
- **Search for a Student's Classes** – Look up class records associated with a specific student.
- **Remove the Last Entry** – Delete the most recently added class record.

## Technologies Used

- **Tauri** – Rust-based framework for building lightweight desktop apps.
- **Rust** – Backend logic for handling file operations and data management.
- **HTML, CSS, JavaScript** – Frontend interface for user interactions.
- **Serde** – Serialization for Rust data structures.
- **dirs** – To locate system directories (e.g., Desktop).

## Usage

1. **Launch the App**
   - Open the application from your installed programs.

2. **Navigate the UI**
   - Click **"Input A New Completed Class"** to add a record.
   - Click **"Read The Full List Of Your Completed Classes"** to view all entries.
   - Click **"Search For A Student's Completed Classes"** to filter records.
   - Click **"Remove Last Inputted Class"** to delete the most recent entry.

## File Handling

- **Data is stored in a local text file**
- The app reads, writes, and modifies this file based on user input.

## Error Handling

The app uses custom error handling for:
- **File Not Found**
- **File Read/Write Issues**
- **Incorrect File Paths**
- **Query Results Not Found**

- ## License
- This project is licensed under the MIT License.
