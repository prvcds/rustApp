# To-Do CLI

A simple command-line to-do list application written in Rust.

## Features

- Add a new task
- List all tasks
- Mark a task as done
- Remove a task
- Search for tasks by description

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/) installed.
2. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/rustApp.git
    ```
3. Navigate to the project directory:
    ```sh
    cd rustApp
    ```
4. Build the project:
    ```sh
    cargo build
    ```

## Usage

Run the application using `cargo run` followed by the desired command and options.

### Commands

- **Add a new task**
    ```sh
    cargo run -- add -d "Task description"
    ```

- **List all tasks**
    ```sh
    cargo run -- list
    ```

- **Mark a task as done**
    ```sh
    cargo run -- done -i 1
    ```

- **Remove a task**
    ```sh
    cargo run -- remove -i 1
    ```

- **Search for tasks by description**
    ```sh
    cargo run -- search -q "query"
    ```

### Help

Display the help message:
```sh
cargo run -- --help
```

Display the version information:
```sh
cargo run -- --version
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
