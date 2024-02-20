use core::fmt;

use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Branch {
    pub name: String,
    pub hash: String,
}

impl Branch {
    pub fn new(name: String) -> Self {
        let hash = hash_name(&name);
        let hash_fmt = &hash[0..6];
        Self {
            name,
            hash: hash_fmt.to_string(),
        }
    }
}

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "    :: [{}] {}", self.hash, self.name)
    }
}

pub struct BranchVec {
    branches: Vec<Branch>,
}

impl BranchVec {
    pub fn new(branches: Vec<Branch>) -> Self {
        Self { branches }
    }

    pub fn filter(&mut self, branch_type: &str) -> BranchVec {
        let branches: Vec<Branch> = self
            .branches
            .clone()
            .into_iter()
            .filter(|branch| branch.name.starts_with(branch_type))
            .collect();
        Self::new(branches)
    }

    pub fn filter_hashes(&self, branch_hashes: Vec<String>) -> Vec<&Branch> {
        self.branches
            .iter()
            .filter(|branch| {
                for hash in &branch_hashes {
                    if branch.hash.starts_with(hash) {
                        return true;
                    }
                }

                false
            })
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.branches.is_empty()
    }
}

impl IntoIterator for BranchVec {
    type Item = Branch;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.branches.into_iter()
    }
}

pub fn hash_name(branch_name: &str) -> String {
    let mut hash = Sha256::new();
    hash.update(branch_name.as_bytes());
    format!("{:x}", hash.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_new() {
        let branch = Branch::new("feature/branch".to_string());
        assert_eq!(branch.name, "feature/branch");
        assert_eq!(branch.hash.len(), 6);
    }

    #[test]
    fn test_branch_vec_filter() {
        let branches = vec![
            Branch::new("feature/branch".to_string()),
            Branch::new("feature/branch2".to_string()),
            Branch::new("bugfix/branch".to_string()),
        ];
        let mut branch_vec = BranchVec::new(branches);
        let filtered = branch_vec.filter("feature");
        assert_eq!(filtered.branches.len(), 2);
    }

    #[test]
    fn test_branch_vec_filter_hashes() {
        let branches = vec![
            Branch::new("feature/branch".to_string()),
            Branch::new("feature/branch2".to_string()),
            Branch::new("bugfix/branch".to_string()),
        ];
        let hash_test = hash_name("feature/branch2");
        let has_test_2 = hash_name("bugfix/branch");
        let branch_vec = BranchVec::new(branches);
        let filtered = branch_vec.filter_hashes(vec![
            hash_test[0..2].to_string(),
            has_test_2[0..4].to_string(),
        ]);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_branch_vec_is_empty() {
        let branches = vec![];
        let branch_vec = BranchVec::new(branches);
        assert!(branch_vec.is_empty());
    }

    #[test]
    fn test_hash_name() {
        let hash = hash_name("feature/branch");
        assert_eq!(hash.len(), 64);
    }
}