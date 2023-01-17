use std::os;

extern crate dirs;

/// Configuration for the application
#[derive(Debug)]
pub struct Config {
    /// base folder path
    pub base_path: String,
    /// pattern to match
    pub patterns: Vec<String>,
    /// pattern to skips
    pub skips: Vec<String>,
}

impl Config {
    /// Create a new Config
    /// # Arguments
    /// * `base_path` - base folder path
    /// * `pattern` - pattern to match
    /// # Returns
    /// * `Config` - new Config
    /// # Example
    /// ```
    /// use find::config::Config;
    /// let config = Config::new("path/to/base/folder", "pattern");
    /// ```
    /// panics if `base_path` or `pattern` is empty
    pub fn new(base_path: &str, patterns: Vec<String>, skips: Vec<String>) -> Self {
        if base_path.is_empty() || patterns.is_empty() {
            panic!("base_path and pattern must not be empty");
        }
        Self {
            base_path: base_path.to_string(),
            patterns,
            skips,
        }
    }

    /// Create a new Config from a pattern, where the default path is the HOME directory
    /// # Arguments
    /// * `pattern` - pattern to match
    pub fn from_pattern(patterns: Vec<String>, skips: Vec<String>) -> Self {
        if patterns.is_empty() {
            panic!("pattern must not be empty");
        }
        Self {
            base_path: dirs::home_dir()
                .expect("home directory not found")
                .to_str()
                .unwrap()
                .to_owned(),
            patterns,
            skips,
        }
    }

    pub fn from_extended_base_path_and_pattern(
        extended_path: &str,
        patterns: Vec<String>,
        skips: Vec<String>,
    ) -> Self {
        if extended_path.is_empty() || patterns.is_empty() {
            panic!("base_path and pattern must not be empty");
        }
        let default_home = dirs::home_dir()
            .expect("home directory not found")
            .to_str()
            .unwrap()
            .to_owned();
        let mut default_path = std::path::Path::new(&default_home);
        let binding = default_path.join(extended_path);
        default_path = binding.as_path();
        Self::new(default_path.to_str().unwrap(), patterns, skips)
    }
}
