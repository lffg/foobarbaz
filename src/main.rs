fn main() {
    log("Hello, world!");
    log("Mid log...");
    log("Now, goodbye!");
}

fn log(d: impl std::fmt::Display) {
    println!("info: {d}");
}
