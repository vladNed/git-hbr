pub mod repo_extension;

use sha2::{Digest, Sha256};

pub fn hash_name(branch_name: &str) -> String {
    let mut hash = Sha256::new();
    hash.update(branch_name.as_bytes());
    format!("{:.6x}", hash.finalize())
}

pub(super) fn omit_branches() -> Vec<&'static str> {
    vec!["master", "develop", "main"]
}
