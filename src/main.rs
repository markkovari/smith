use std::collections::{HashMap, HashSet};

use crate::config::Config;
use walkdir::WalkDir;

mod config;

/// Main function
fn main() {
    let config = Config::from_extended_base_path_and_pattern(
        "DEV",
        vec!["DELETE".to_owned()],
        vec![
            ".git".to_owned(),
            ".idea".to_owned(),
            ".DS_Store".to_owned(),
            "node_modules".to_owned(),
        ],
    );

    let mut folders: HashSet<String> = HashSet::new();

    for entry in WalkDir::new(config.base_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path().to_str().unwrap();
        if entry.file_type().is_dir()
            && !config.skips.iter().any(|skip| path.contains(skip))
            && config.patterns.iter().any(|pattern| path.contains(pattern))
            && !folders.iter().any(|collected| path.starts_with(collected))
        {
            folders.insert(path.to_owned());
        }
    }
    println!("{:?}", folders);
}
