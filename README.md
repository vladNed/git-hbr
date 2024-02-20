# RIT (Rust Git Assistant)

RIT is a simple git assistant written in Rust. It is a command line tool that helps you to manage your git repositories.
Written out of curiosity on what can be done to better manage git branches, this project was also inspired from how a
user usually deletes containers in Docker.
Using the simplicity of hashes, RIT is able to manage branches in a unique way.

## Installation

### Locally

Using `cargo`:
```bash
cargo install --path <path_to_rit_cloned_repo>
```

> Note: This is still experimental and was not released to crates.io or tested on all platforms.

## Test

Using `cargo`:
```bash
cargo test
```

## Usage

After the installation, you can use the `rit` command to manage your git repositories branches.

Within a git repository, you can use the following commands:

- `rit list` - List all branches
- `rit list <branch_type>` - List all branches of a specific type. The branch type can be `feature`, `bugfix` or `hotfix`.
- `rit delete <branch_hash>` - Delete a branch using the branch hash. A partial hash can be also used instead of the full hash.
