use anyhow::{anyhow, Result};
use cat_file::pretty_cat_file;
use clap::Parser;
use cli::{Cli, Commands};
use hash_object::hash_object_write;
use ls_tree::ls_tree_name_only;
use std::fs;

mod cat_file;
mod cli;
mod hash_object;
mod ls_tree;
mod object;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            init()?;
        }
        Commands::CatFile {
            pretty_print,
            blob_sha,
        } => {
            if !pretty_print {
                return Err(anyhow!("The `-p` flag is required"));
            }

            pretty_cat_file(blob_sha)?;
        }
        Commands::HashObject { write, path } => {
            if !write {
                return Err(anyhow!("The `-w` flag is required"));
            }
            hash_object_write(path)?;
        }
        Commands::LsTree { name_only, tree } => {
            if !name_only {
                return Err(anyhow!("The `--name-only` flag is required"));
            }
            ls_tree_name_only(tree)?;
        }
    }

    Ok(())
}

fn init() -> Result<()> {
    fs::create_dir(".git")?;
    fs::create_dir(".git/objects")?;
    fs::create_dir(".git/refs")?;
    fs::write(".git/HEAD", "ref: refs/heads/master\n")?;
    println!("Initialized git directory");
    Ok(())
}
