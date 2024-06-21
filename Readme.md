# Client Application in Rust with GTK-rs

## Description

This project is a client application developed in Rust using the `gtk-rs` library to create a GUI. The application allows users to browse directories, select a directory to index, and perform keyword searches within the indexed document corpus. By default, the search engine loads the last indexed directory, enabling immediate search functionality on the previously indexed folder. There are still many features to implement, including the ability to choose an index file to load for the search engine.

## Features

- **Directory Browsing**: Users can browse their file system to select a directory for indexing.
- **Indexing**: The application indexes the selected directory and its subdirectories.
- **Keyword Search**: Users can perform keyword searches within the indexed documents.
- **Default Loading**: The search engine loads the last indexed directory by default, allowing for quick searches on the most recent data.

## Dependencies

To run this project, you need to install the following dependencies:

```toml
[dependencies]
env_logger = "0.10.1"
gtk = { version = "0.7.3", package = "gtk4", features = ["v4_12"] }
log = "0.4.20"
back_end_indexer = { git = "https://github.com/DevprojectEkla/back_end_indexer.git", branch = "main" }
```

## Installation

1. **Clone the Repository**:
    ```bash
    git clone https://github.com/DevprojectEkla/RustIndexer
    cd RustIndexer 
    ```

2. **Build the Project**:
    ```bash
    cargo build
    ```

3. **Run the Application**:
    ```bash
    cargo run
    ```

## Usage

1. **Browsing and Indexing**:
    - Launch the application.
    - Use the file dialog to browse and select the directory you wish to index.
    - Click the "Index" button to start the indexing process for the selected directory and its subdirectories.

2. **Keyword Search**:
    - Enter the keyword(s) you wish to search for in the search bar.
    - Click the "Search" button to perform the search within the indexed documents.
    - The results will be displayed in the results pane.

3. **Default Loading**:
    - On startup, the application automatically loads the last indexed directory.
    - This feature allows for quick searches on the most recent dataset without needing to re-index.

## Roadmap

- **Custom Index Loading**: Implement the ability to choose and load a specific index file for the search engine.
- **Advanced Search Options**: Add more search filters and options to refine search results.
- **Improved Indexing**: Optimize the indexing process for better performance with large directories.
- **UI Enhancements**: Improve the user interface for a better user experience.

## Contribution

Contributions are welcome! Please fork the repository and submit pull requests for any enhancements or bug fixes.

## License

This project is licensed under the GPL-3.0 License. See the [LICENSE](https://github.com/DevprojectEkla/RustIndexer/blob/main/LICENSE) file for details.


