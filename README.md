# NovaVM

NovaVM is a virtual machine written in Rust for Novalang

## Project Structure

### Key Files and Directories

- **Cargo.toml**: Contains the metadata and dependencies for the Rust project.
- **src/**: Contains the source code for the project.
  - **bin/**: Contains the binary files for the project.
    - **machine/**: Contains the main entry point for the machine binary.
      - **main.rs**: The main file for the machine binary.
    - **preprocessor/**: Contains the main entry pount for the preprocessor binary.
      - **main.rs**: The main file for the preprocessor binary.
  - **fetch.rs**: Handles fetching operations.
  - **lib.rs**: The main library file for the project.
  - **opcode.rs**: Contains the opcode definitions and related logic.
- **target/**: Contains the compiled output and build artifacts.

## Building the Project

To build the project, run the following command:

```sh
cargo build
```
