use globwalk::GlobWalkerBuilder;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct YadmConfig {
    pub file: FileConfig,
}

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
            base_config_directories: vec![
                "~/.config".into(),
                // "~/.local/state".into(),
                // "~/.local/share".into(),
            ],
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
            ],
        }
    }
}

impl FileConfig {
    fn expand(path: &str) -> PathBuf {
        shellexpand::tilde(path).to_string().into()
    }

    pub fn load_matching(&self) -> Vec<PathBuf> {
        let mut results = Vec::new();
        // let patterns = ["**/*.conf", "**/*.toml"];
        let patterns = ["**/*"];

        for dir in &self.config_directories {
            let dir = Self::expand(dir);

            if !dir.exists() {
                continue;
            }

            let walker = GlobWalkerBuilder::from_patterns(&dir, &patterns)
                .build()
                .unwrap();

            for entry in walker.filter_map(Result::ok) {
                results.push(entry.path().to_path_buf());
            }
        }

        for base_dir in &self.base_config_directories {
            let base_dir = Self::expand(base_dir);

            if !base_dir.exists() {
                continue;
            }

            let walker = GlobWalkerBuilder::from_patterns(&base_dir, &patterns)
                .build()
                .unwrap();

            for entry in walker.filter_map(Result::ok) {
                results.push(entry.path().to_path_buf());
            }
        }

        for file in &self.single_files {
            let file = Self::expand(file);

            if file.exists() {
                results.push(file);
            }
        }

        results
    }
}
