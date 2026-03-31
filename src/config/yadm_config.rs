use super::FileConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct YadmConfig {
    pub file: FileConfig,
}
