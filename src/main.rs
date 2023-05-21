use clap::{Parser, Subcommand};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{
    fs,
    io::{Read, Write},
    process,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
    HashObject {
        /// Actually write the object into the object database
        #[arg(short)]
        write: bool,

        ///
        file_name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            init();
        }
        Commands::CatFile {
            pretty_print,
            blob_sha,
        } => {
            if !pretty_print {
                eprintln!("The `-p` flag is required");
                process::exit(1);
            }

            cat_file(blob_sha);
        }
        Commands::HashObject { write, file_name } => {
            if !write {
                eprintln!("The `-w` flag is required");
                process::exit(1);
            }
            hash_object(file_name);
        }
    }
}

fn init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
    println!("Initialized git directory")
}

fn cat_file(blob_sha: &str) {
    let dir_name = &blob_sha[..2];
    let file_name = &blob_sha[2..];
    let path = format!("./.git/objects/{dir_name}/{file_name}");
    let data = fs::read(path).unwrap();
    let mut dec = ZlibDecoder::new(data.as_slice());
    let mut result = String::new();
    dec.read_to_string(&mut result).unwrap();
    let header_index = result.find('\0').unwrap();
    print!("{}", &result[header_index + 1..]);
}

fn hash_object(file_name: &str) {
    let content = fs::read_to_string("./".to_string() + file_name).unwrap();
    let header = format!("blob {}\x00", content.len());
    let store = format!("{header}{content}");
    let mut hasher = Sha1::new();
    hasher.update(store.as_bytes());
    let res = hasher.finalize();
    let blob_sha = hex::encode(res);
    let dir_name = &blob_sha[..2];
    let file_name = &blob_sha[2..];
    fs::create_dir(format!("./.git/objects/{dir_name}")).unwrap();
    let path = format!("./.git/objects/{dir_name}/{file_name}");
    let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
    enc.write_all(store.as_bytes()).unwrap();
    let enc_store = enc.finish().unwrap();
    fs::write(path, enc_store).unwrap();
    print!("{blob_sha}");
}
