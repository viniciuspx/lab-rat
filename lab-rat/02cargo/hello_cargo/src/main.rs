// Wow! Same-same but different, in this bit we used cargo to start the project 

// Cargo is Rust’s build system and package manager

fn main() {
    println!("Hello, world!");
}

// TOML (Tom’s Obvious, Minimal Language)
// Format, which is Cargo’s configuration format.
// In Rust, packages of code are referred to as crates.
// Cargo.lock. This file keeps track of the exact versions of dependencies in your project. 

// Cargo main cmds

// cargo build
// cargo build --release
// cargo run
// cargo check


// Simple cargo project view
//.
//├── Cargo.lock
//├── Cargo.toml
//├── src
//│   └── main.rs
//└── target
//    ├── CACHEDIR.TAG
//    └── debug
//        ├── build
//        ├── deps
//        ├── examples
//        ├── hello_cargo
//        ├── hello_cargo.d
//        └── incremental