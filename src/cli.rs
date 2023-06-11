use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialise a new repository
    Init,

    /// Provide content or type and size information for repository objects
    CatFile {
        /// Pretty print the object
        #[arg(short)]
        pretty_print: bool,

        /// The object to cat
        blob_sha: String,
    },

    /// Compute object ID and optionally creates a blob from a file
    HashObject {
        /// Actually write the object into the object database
        #[arg(short)]
        write: bool,

        /// The file to hash
        path: PathBuf,
    },

    /// Lists the contents of a given tree object
    LsTree {
        /// List only filenames
        #[arg(long)]
        name_only: bool,

        /// The id of a tree
        tree: String,
    },

    /// Creates a tree object
    WriteTree,

    CommitTree {
        #[arg(short)]
        parent_sha: String,

        #[arg(short)]
        message: String,

        tree_sha: String,
    },
}
