use git2::Repository;
use structopt::StructOpt;

mod cli;
mod core;

use core::repo_extension::RepositoryExtension;

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

    let repo_ext = RepositoryExtension::new(repo);
    if opt.listing {
        repo_ext.list(None);
    } else {
        repo_ext.delete_branch(opt.branch_hashes);
    }
}
