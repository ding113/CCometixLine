use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub default_context_limit: u32,
    #[serde(rename = "models")]
    pub model_entries: Vec<ModelEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEntry {
    pub pattern: String,
    pub display_name: String,
    pub context_limit: u32,
}

impl ModelConfig {
    /// Load model configuration from TOML file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: ModelConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Load model configuration with fallback locations
    pub fn load() -> Self {
        // Try loading from user config directory first
        let config_paths = [
            dirs::home_dir().map(|d| d.join(".claude").join("ccline").join("models.toml")),
            Some(Path::new("models.toml").to_path_buf()),
        ];

        for path in config_paths.iter().flatten() {
            if path.exists() {
                if let Ok(config) = Self::load_from_file(path) {
                    return config;
                }
            }
        }

        // Fallback to default configuration if no file found
        Self::default()
    }

    /// Get context limit for a model based on ID pattern matching
    /// Checks external config first, then falls back to built-in config
    pub fn get_context_limit(&self, model_id: &str) -> u32 {
        let model_lower = model_id.to_lowercase();

        // First check external config if it exists
        if let Ok(external_config) = Self::load_external_config() {
            for entry in &external_config.model_entries {
                if model_lower.contains(&entry.pattern.to_lowercase()) {
                    return entry.context_limit;
                }
            }
        }

        // Fall back to built-in config
        for entry in &self.model_entries {
            if model_lower.contains(&entry.pattern.to_lowercase()) {
                return entry.context_limit;
            }
        }

        self.default_context_limit
    }

    /// Get display name for a model based on ID pattern matching
    /// Checks external config first, then falls back to built-in config
    /// Returns None if no match found (should use fallback display_name)
    pub fn get_display_name(&self, model_id: &str) -> Option<String> {
        let model_lower = model_id.to_lowercase();

        // First check external config if it exists
        if let Ok(external_config) = Self::load_external_config() {
            for entry in &external_config.model_entries {
                if model_lower.contains(&entry.pattern.to_lowercase()) {
                    return Some(entry.display_name.clone());
                }
            }
        }

        // Fall back to built-in config
        for entry in &self.model_entries {
            if model_lower.contains(&entry.pattern.to_lowercase()) {
                return Some(entry.display_name.clone());
            }
        }

        None
    }

    /// Load external configuration file only
    fn load_external_config() -> Result<Self, Box<dyn std::error::Error>> {
        let config_paths = [
            dirs::home_dir().map(|d| d.join(".claude").join("ccline").join("models.toml")),
            Some(Path::new("models.toml").to_path_buf()),
        ];

        for path in config_paths.iter().flatten() {
            if path.exists() {
                return Self::load_from_file(path);
            }
        }

        Err("No external config file found".into())
    }

    /// Create default model configuration file with minimal template
    pub fn create_default_file<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
        // Create a minimal template config (not the full fallback config)
        let template_config = Self {
            default_context_limit: 200_000,
            model_entries: vec![], // Empty - just provide the structure
        };

        let toml_content = toml::to_string_pretty(&template_config)?;

        // Add comments and examples to the template
        let template_content = format!(
            "# CCometixLine Model Configuration\n\
             # This file defines model display names and context limits for different LLM models\n\
             # File location: ~/.claude/ccline/models.toml\n\
             \n\
             {}\n\
             \n\
             # Model configurations\n\
             # Each [[models]] section defines a model pattern and its properties\n\
             # Order matters: first match wins, so put more specific patterns first\n\
             \n\
             # Example of how to add new models:\n\
             # [[models]]\n\
             # pattern = \"glm-4.5\"\n\
             # display_name = \"GLM-4.5\"\n\
             # context_limit = 128000\n",
            toml_content.trim()
        );

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, template_content)?;
        Ok(())
    }
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            default_context_limit: 200_000,
            model_entries: vec![
                // 1M context models (put first for priority matching)
                ModelEntry {
                    pattern: "[1m]".to_string(),
                    display_name: "Sonnet 4 1M".to_string(),
                    context_limit: 1_000_000,
                },
                ModelEntry {
                    pattern: "claude-sonnet-4".to_string(),
                    display_name: "Sonnet 4".to_string(),
                    context_limit: 200_000,
                },
                ModelEntry {
                    pattern: "claude-4-sonnet".to_string(),
                    display_name: "Sonnet 4".to_string(),
                    context_limit: 200_000,
                },
                ModelEntry {
                    pattern: "claude-4-opus".to_string(),
                    display_name: "Opus 4".to_string(),
                    context_limit: 200_000,
                },
                ModelEntry {
                    pattern: "sonnet-4".to_string(),
                    display_name: "Sonnet 4".to_string(),
                    context_limit: 200_000,
                },
                ModelEntry {
                    pattern: "claude-3-7-sonnet".to_string(),
                    display_name: "Sonnet 3.7".to_string(),
                    context_limit: 200_000,
                },
                // Third-party models
                ModelEntry {
                    pattern: "glm-4.5".to_string(),
                    display_name: "GLM-4.5".to_string(),
                    context_limit: 128_000,
                },
                ModelEntry {
                    pattern: "kimi-k2-turbo".to_string(),
                    display_name: "Kimi K2 Turbo".to_string(),
                    context_limit: 128_000,
                },
                ModelEntry {
                    pattern: "kimi-k2".to_string(),
                    display_name: "Kimi K2".to_string(),
                    context_limit: 128_000,
                },
                ModelEntry {
                    pattern: "qwen3-coder".to_string(),
                    display_name: "Qwen Coder".to_string(),
                    context_limit: 256_000,
                },
            ],
        }
    }
}
