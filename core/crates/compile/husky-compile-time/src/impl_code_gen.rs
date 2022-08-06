use std::path::Path;

use crate::*;
use husky_rust_code_gen::cargo_toml_content;

impl HuskyComptime {
    pub fn cargo_toml_content(&self, crate_entrance: FilePtr, husky_dir: &str) -> String {
        cargo_toml_content(self, crate_entrance, husky_dir)
    }
}
