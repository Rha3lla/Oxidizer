use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tool: ToolConfig,
    pub adapters: Adapters,
    pub thresholds: Thresholds,
}

#[derive(Debug, Deserialize)]
pub struct ToolConfig {
    pub profile: String,
    pub output: String,
    pub fail_on: String,
}

#[derive(Debug, Deserialize)]
pub struct Adapters {
    pub rudra: bool,
    pub geiger: bool,
    pub semgrep: bool,
    pub audit: bool,
}

#[derive(Debug, Deserialize)]
pub struct Thresholds {
    pub unsafe_ratio_max: f64,
}

#[allow(clippy::derivable_impls)]
impl Default for Config {
    fn default() -> Self {
        Config {
            tool: ToolConfig::default(),
            adapters: Adapters::default(),
            thresholds: Thresholds::default(),
        }
    }
}
impl Default for ToolConfig {
    fn default() -> Self {
        ToolConfig {
            profile: String::from("high-assurance"),
            output: String::from("sarif"),
            fail_on: String::from("error"),
        }
    }
}

impl Default for Thresholds {
    fn default() -> Self {
        Thresholds {
            unsafe_ratio_max: 0.05,
        }
    }
}


impl Default for Adapters {
    fn default() -> Self {
        Adapters {
            rudra: true,
            geiger: true,
            semgrep: true,
            audit: true,
        }
    }
}

impl Config {
    pub fn load(path: &std::path::Path) -> Config {
        //while let loop 
        //Walk up from path toward the filesystem root looking for rust-sa.toml
        //If found, read it and parse it into a Config using the toml crate
        //If not found, or if parsing fails, return Config::default()
        //returns Result<String, Error>
        let mut current_path = Some(path);
        while let Some(path) = current_path {
            let config_path = path.join("rust-sa.toml");
            if config_path.exists() {
                match std::fs::read_to_string(&config_path) {
                    Ok(contents) => match toml::from_str::<Config>(&contents) {
                        Ok(config) => return config,
                        Err(err) => {
                            eprintln!("Failed to parse config file {}: {}", config_path.display(), err);
                            return Config::default();
                        }
                    },
                    Err(err) => {
                        eprintln!("Failed to read config file {}: {}", config_path.display(), err);
                        return Config::default();
                    }
                }
            }
            current_path = path.parent();
        }
        Config::default()

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        // Test that the default config has the expected values
        let config = Config::default();
        assert_eq!(config.tool.profile, "high-assurance");
        assert_eq!(config.tool.output, "sarif");
        assert_eq!(config.tool.fail_on, "error");
    }

    #[test]
    fn test_missing_config_uses_defaults () {
        // call Config::load() with a path that doesn't have a rust-sa.toml (like the root of the filesystem) 
        //and assert you get back a valid config with default values rather than a panic.
        let config = Config::load(std::path::Path::new("/"));
        assert_eq!(config.tool.profile, "high-assurance"); 
        assert_eq!(config.tool.output, "sarif");
        assert_eq!(config.tool.fail_on, "error");        
    }
}