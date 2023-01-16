use crate::config::Config;

mod config;

/// Main function
fn main() {
    let config = Config::new("path/to/base/folder", "pattern");
    println!("Hello, Shoot me in the head!");
}
