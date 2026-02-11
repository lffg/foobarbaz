//! This is my main binary.

fn main() {
    log("Starting...");
    log("Hello, world!");
    log("Mid log...");
    log("Now, goodbye!");
}

fn log(d: impl std::fmt::Display) {
    println!("info: {d}");
}
