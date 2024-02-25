# git-hbr (Hashed Branch GIT Supplement)

`git-hbr` is a simple git assistant written in Rust. It is a command line tool that helps you to manage your git repositories.
Written out of curiosity on what can be done to better manage git branches, this project was also inspired from how a
user usually deletes containers in Docker.
Using the simplicity of hashes, `git-hbr` is able to manage branches in a unique way.

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

After the installation, you can use the `git hbr` command to manage your git repositories branches.

### List

Example:
```bash
$ git hbr -l
   :: [omitted] develop
   :: [240e4a] feature/c-12345/asd
   :: [48277c] feature/c-s92ls/fgas123
   :: [omitted] main
```

### Delete

Example:
```bash
$ git hbr 240 482
Deleting following branches:
   - feature/c-12345/asd
   - feature/c-s92ls/fgas123
✔ Are you sure you want to delete these branches? · yes
Deleting branch: feature/c-12345/asd
Deleting branch: feature/c-s92ls/fgas123
```

