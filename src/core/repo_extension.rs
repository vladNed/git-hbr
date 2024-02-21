use dialoguer::{theme::ColorfulTheme, Confirm};

pub struct RepositoryExtension {
    repo: git2::Repository,
}

impl RepositoryExtension {
    pub fn new(repo: git2::Repository) -> Self {
        Self { repo }
    }

    fn get_local_branches(&self) -> git2::Branches {
        self.repo.branches(Some(git2::BranchType::Local)).unwrap()
    }

    pub fn list(&self, branch_type: Option<String>) {
        let repo_branches = self.get_local_branches();
        let omit_branches = super::omit_branches();
        let filter = match branch_type {
            Some(filter_value) => filter_value,
            None => String::from("all"),
        };

        for branch in repo_branches {
            let (branch, _) = branch.unwrap();
            let branch_name = branch.name().unwrap().unwrap();

            if filter != "all" && !branch_name.starts_with(&filter) {
                continue;
            }
            let branch_name_hash = if omit_branches.contains(&branch_name) {
                String::from("omitted")
            } else {
                super::hash_name(branch_name)
            };
            println!("   :: [{}] {}", branch_name_hash, branch_name);
        }
    }

    pub fn delete_branch(&self, branch_name_hashes: Vec<String>) {
        let repo_branches = self.get_local_branches();
        let omit_branches = super::omit_branches();
        let branches_to_delete: Vec<_> = repo_branches
            .filter(|branch| {
                let (branch, _) = branch.as_ref().unwrap();
                let branch_name = branch.name().unwrap().unwrap();
                if omit_branches.contains(&branch_name) {
                    return false;
                }
                let branch_name_hash = super::hash_name(branch_name);
                for hash in &branch_name_hashes {
                    if branch_name_hash.starts_with(hash) {
                        return true;
                    }
                }

                false
            })
            .collect();

        if branches_to_delete.is_empty() {
            println!("== Could not find any selected branches ==");
            return;
        }

        println!("Deleting following branches:");
        for branch in &branches_to_delete {
            let (branch, _) = branch.as_ref().unwrap();
            let branch_name = branch.name().unwrap().unwrap();
            println!("   - {}", branch_name);
        }

        if !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Are you sure you want to delete these branches?")
            .interact()
            .unwrap()
        {
            println!("Aborting...");
            return;
        }

        for branch in branches_to_delete {
            let (mut branch, _) = branch.unwrap();
            println!("Deleting branch: {}", branch.name().unwrap().unwrap());
            branch.delete().unwrap_or_else(|e| {
                println!(
                    "Failed to delete branch: {} :: {}",
                    branch.name().unwrap().unwrap(),
                    e
                );
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::*;

    #[test]
    fn test_hash_name() {
        let hash = hash_name("test");
        assert_eq!(hash, "9f86d0");
    }
}
