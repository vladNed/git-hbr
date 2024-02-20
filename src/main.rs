use git2::Repository;
use structopt::StructOpt;

mod cli;
mod core;

fn main() {
    let opt = cli::Opt::from_args();
    let path = std::env::current_dir().unwrap();
    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(_) => {
            println!("== Failed to open repo ==");
            return;
        }

    };

    let mut branches = core::load_branches(repo.branches(Some(git2::BranchType::Local)).unwrap());
    if branches.is_empty() {
        println!("== No branches found ==");
        return;
    }

    match opt {
        cli::Opt::List { branch_type } => {
            if let Some(ref branch_type) = branch_type {
                branches = branches.filter(branch_type);
            }

            for branch in branches {
                println!("{}", branch);
            }
        }
        cli::Opt::Delete { branch_hashes } => {
            let branches_to_delete = branches.filter_hashes(branch_hashes);
            for branch in branches_to_delete {
                println!("Deleting branch: {}", branch.name);
            }
        }
    }
}