pub use super::SampleNameConfig;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use xmodits_lib::export::Format;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct SampleRippingConfig {
    pub destination: PathBuf,
    pub self_contained: bool,
    pub folder_max_depth: u8,
    pub strict: bool,
    pub worker_threads: usize,
    pub exported_format: Format,
}

impl Default for SampleRippingConfig {
    fn default() -> Self {
        Self {
            destination: default_dir(),
            self_contained: true,
            folder_max_depth: 4,
            strict: true,
            exported_format: Default::default(),
            worker_threads: 0,
        }
    }
}

fn default_dir() -> PathBuf {
    let fallback = || std::env::current_dir().unwrap_or_default();
    dirs::download_dir().unwrap_or_else(fallback)
}
