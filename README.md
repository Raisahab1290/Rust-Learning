**Hey!!! Everyone,I am learning Rust.**

# 📘 Rust Learning Repository

This repository documents my structured learning journey of the Rust programming language,
based on the official Rust documentation and hands-on practice.

Purpose of this repository:
- Track Rust learning progress
- Practice concepts through small, focused examples
- Make the codebase easy to understand for fresh developers reviewing it

=====================================================================

REPOSITORY STRUCTURE, FOLDER EXPLANATION & RUN COMMANDS
(All in one place for easy understanding)

Rust-Learning/
├── lect1/
│   │
│   ├── Hello_World/
│   │   ├── main.rs
│   │   │
│   │   │  Description:
│   │   │  - A basic Rust program compiled and run using rustc
│   │   │  - Used to understand:
│   │   │      • Rust program structure
│   │   │      • Entry point (main function)
│   │   │      • Manual compilation and execution
│   │   │
│   │   │  Run:
│   │   │      rustc main.rs
│   │   │      ./main
│   │
│   └── hello_cargo/
│       ├── Cargo.toml
│       ├── Cargo.lock
│       └── src/
│           └── main.rs
│
│       Description:
│       - A Rust project created using Cargo
│       - Used to understand:
│           • Standard Rust project structure
│           • Build & run workflow
│           • Dependency management
│
│       Run:
│           cd lect1/hello_cargo
│           cargo run
│
├── .gitignore
│   - Ignores build artifacts, binaries, and generated files
│
└── README.md
    - Documentation of the learning journey

=====================================================================

LEARNING LOG (CONTINUOUSLY UPDATED)

Lecture 1: Introduction to Rust
--------------------------------

Example 1: Hello World (Without Cargo)

Code:
    fn main() {
        println!("Hello, world!");
    }

What this teaches:
- main is the program entry point
- println! is a macro
- Basic Rust syntax and structure

Run:
    rustc main.rs
    ./main

--------------------------------

Example 2: Hello World (Using Cargo)

Code:
    fn main() {
        println!("Hello from Cargo!");
    }

What this teaches:
- Cargo manages builds and execution
- Cargo.toml defines project metadata
- cargo run compiles and runs the project

Run:
    cd lect1/hello_cargo
    cargo run

=====================================================================

NOTES
- This README will grow as learning progresses
- New lectures will be appended below
- Older examples may be refined later

REMINDER
If a file can be generated again, it does NOT belong in Git.