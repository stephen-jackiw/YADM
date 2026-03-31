use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileConfig {
    pub config_directories: Vec<String>,
    pub base_config_directories: Vec<String>,
    pub single_files: Vec<String>,
    pub excluded_file_types: Vec<String>,
    pub excluded_directories: Vec<String>,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            config_directories: vec![
                "/etc/ssh".into(),
                "/lib/postgresql".into(),
                "/etc/fish".into(),
            ],
            base_config_directories: vec!["~/.config".into()],
            single_files: vec![
                "~/.m2/settings.xml".into(),
                "~/.npmrc".into(),
                "~/.zshrc".into(),
            ],
            excluded_file_types: vec![".log".into(), ".bin".into(), ".pyi".into()],
            excluded_directories: vec![
                "~/.config/coc".into(),
                "~/.config/discord".into(),
                "~/.config/browsh".into(),
                "~/.config/VSCodium".into(),
                "~/.config/pgadmin4".into(),
                "~/.config/cef_user_data".into(),
                "~/.config/obsidian/Cache".into(),
                "~/.config/Insomnia".into(),
                "~/.config/BraveSoftware".into(),
                "cache".into(), // note: fixed typo "chache"
                "Cache".into(), // note: fixed typo "Chache"
            ],
        }
    }
}
