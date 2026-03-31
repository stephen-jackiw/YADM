use crate::config::FileConfig;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn expand(path: &str) -> PathBuf {
    shellexpand::tilde(path).to_string().into()
}

fn is_excluded_dir(config: &FileConfig, path: &Path) -> bool {
    config
        .excluded_directories
        .iter()
        .any(|excl| path.starts_with(expand(excl)))
}

fn is_excluded_extension(config: &FileConfig, path: &Path) -> bool {
    let name = path.to_string_lossy();
    config
        .excluded_file_types
        .iter()
        .any(|ext| name.ends_with(ext.as_str()))
}

fn walk_dir(config: &FileConfig, dir: &Path, results: &mut Vec<PathBuf>) {
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| !is_excluded_dir(config, e.path()))
        .filter_map(Result::ok)
    {
        let path = entry.into_path();
        if path.is_file() && !is_excluded_extension(config, &path) {
            results.push(path);
        }
    }
}

pub fn load_matching(config: &FileConfig) -> Vec<PathBuf> {
    let mut results = Vec::new();

    for dir in &config.config_directories {
        let dir = expand(dir);
        if dir.exists() {
            walk_dir(config, &dir, &mut results);
        }
    }

    for base_dir in &config.base_config_directories {
        let base_dir = expand(base_dir);
        if base_dir.exists() {
            walk_dir(config, &base_dir, &mut results);
        }
    }

    for file in &config.single_files {
        let file = expand(file);
        if file.exists() {
            results.push(file);
        }
    }

    results
}
