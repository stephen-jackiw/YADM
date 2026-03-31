use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use walkdir::WalkDir;

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
                "~/.config/VSCodium".into(),
                "~/.config/pgadmin4".into(),
                "~/.config/cef_user_data".into(),
                "~/.config/obsidian/Cache".into(),
                "~/.config/Insomnia".into(),
                "~/.config/BraveSoftware".into(),
                "chache".into(),
                "Chache".into(),
            ],
        }
    }
}

impl FileConfig {
    fn expand(path: &str) -> PathBuf {
        shellexpand::tilde(path).to_string().into()
    }

    fn is_excluded_dir(&self, path: &PathBuf) -> bool {
        self.excluded_directories.iter().any(|excl| {
            let excl = Self::expand(excl);
            path.starts_with(&excl)
        })
    }

    fn is_excluded_extension(&self, path: &PathBuf) -> bool {
        let name = path.to_string_lossy();
        self.excluded_file_types
            .iter()
            .any(|ext| name.ends_with(ext.as_str()))
    }

    fn walk_dir(&self, dir: &PathBuf, results: &mut Vec<PathBuf>) {
        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_entry(|e| !self.is_excluded_dir(&e.path().to_path_buf()))
            .filter_map(Result::ok)
        {
            let path = entry.path().to_path_buf();
            if path.is_file() && !self.is_excluded_extension(&path) {
                results.push(path);
            }
        }
    }

    pub fn load_matching(&self) -> Vec<PathBuf> {
        let mut results = Vec::new();

        for dir in &self.config_directories {
            let dir = Self::expand(dir);
            if dir.exists() {
                self.walk_dir(&dir, &mut results);
            }
        }

        for base_dir in &self.base_config_directories {
            let base_dir = Self::expand(base_dir);
            if base_dir.exists() {
                self.walk_dir(&base_dir, &mut results);
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
