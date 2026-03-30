use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct YadmConfig {
    pub file: FileConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileConfig {
    // directories for single apps or features
    pub config_directories: Vec<String>,
    // directories for many apps
    pub base_config_directories: Vec<String>,
    // for adhoc config file that doesnt have its own directory
    pub single_files: Vec<String>,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            config_directories: vec![
                "/etc/ssh".into(),
                "/lib/postgresql".into(),
                "/etc/fish".into(),
            ],
            base_config_directories: vec![
                "~/.config".into(),
                "~/.local/state".into(),
                "~/.local/share".into(),
            ],
            single_files: vec![
                "~/.m2/settings.xml".into(),
                "~/.npmrc".into(),
                "~/.zshrc".into(),
            ],
        }
    }
}
