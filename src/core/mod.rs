mod branch;

pub use branch::hash_name;

pub fn load_branches(branches: git2::Branches) -> branch::BranchVec {
    let mut branches_vec: Vec<branch::Branch> = vec![];
    for branch in branches {
        let (branch, _) = branch.unwrap();
        let name = branch.name().unwrap().unwrap();
        let branch = branch::Branch::new(name.to_string());
        branches_vec.push(branch);
    }

    branch::BranchVec::new(branches_vec)
}
