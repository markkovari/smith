use crate::config::Config;

mod config;

/// Main function
fn main() {
    let config = Config::new("/Users/markkovari/DEV", vec!["pattern".to_owned()]);
    let pattern_only =
        Config::from_extended_base_path_and_pattern("DEV", vec!["pattern".to_owned()]);
    println!("{:?}", config);
    println!("{:?}", pattern_only);
    println!("Hello, Shoot me in the head!");
}
