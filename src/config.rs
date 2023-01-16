use std::os;

extern crate dirs;

/// Configuration for the application
#[derive(Debug)]
pub struct Config {
    /// base folder path
    base_path: String,
    /// pattern to match
    pattern: String,
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
    pub fn new(base_path: &str, pattern: &str) -> Self {
        if base_path.is_empty() || pattern.is_empty() {
            panic!("base_path and pattern must not be empty");
        }
        Self {
            base_path: base_path.to_string(),
            pattern: pattern.to_string(),
        }
    }

    /// Create a new Config from a pattern, where the default path is the HOME directory
    /// # Arguments
    /// * `pattern` - pattern to match
    pub fn from_pattern(pattern: &str) -> Self {
        if pattern.is_empty() {
            panic!("pattern must not be empty");
        }
        Self {
            base_path: dirs::home_dir()
                .expect("home directory not found")
                .to_str()
                .unwrap()
                .to_owned(),
            pattern: pattern.to_string(),
        }
    }

    pub fn from_extended_base_path_and_pattern(extended_path: &str, pattern: &str) -> Self {
        if extended_path.is_empty() || pattern.is_empty() {
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
        Self::new(default_path.to_str().unwrap(), pattern)
    }
}
