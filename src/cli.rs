use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "git hbr", about = "Manage local branches using hash identifiers")]
pub struct Opt {

    pub branch_hashes: Vec<String>,

    #[structopt(short = "l", long = "list", help = "List all local branches with their hash identifiers")]
    pub listing: bool,
}
