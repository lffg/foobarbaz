fn main() {
    log("Hello, world!");
    log("Now, goodbye!");
}

fn log(d: impl std::fmt::Display) {
    println!("info: {d}");
}
