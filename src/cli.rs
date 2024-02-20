use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rit", about = "Rust Git Assistant")]
pub enum Opt {
    #[structopt(name = "list", about = "List all local branches")]
    List {
        #[structopt(short, long)]
        branch_type: Option<String>,
    },

    #[structopt(name = "delete", about = "Delete a local branch")]
    Delete {
        #[structopt(
            name = "branch hashes",
            required = true,
            min_values = 1,
            about = "Branch hashes to delete"
        )]
        branch_hashes: Vec<String>,
    },
}
