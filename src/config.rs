/// Configuration for the application
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
}
