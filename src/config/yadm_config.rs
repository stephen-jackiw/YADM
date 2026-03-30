use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct YadmConfig {
    pub file: FileConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileConfig {
    pub config_directories: Vec<String>,
    pub base_config_directories: Vec<String>,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            config_directories: vec!["/etc/ssh".into()],
            base_config_directories: vec!["~/.config".into()],
        }
    }
}
