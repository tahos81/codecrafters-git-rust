use flate2::read::ZlibDecoder;
use hex;
use sha1::{Digest, Sha1};
use std::env;
use std::fs;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();
    match command {
        "init" => init(),
        "cat-file" => {
            let blob_sha = args[3].as_str();
            cat_file(blob_sha);
        }
        "hash-object" => {
            let file_name = args[3].as_str();
            hash_object(file_name);
        }
        _ => println!("unknown command: {}", command),
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
    let path = format!("./.git/objects/{}/{}", dir_name, file_name);
    let data = fs::read(path).unwrap();
    let mut dec = ZlibDecoder::new(data.as_slice());
    let mut result = String::new();
    dec.read_to_string(&mut result).unwrap();
    let header_index = result.find('\0').unwrap();
    print!("{}", &result[header_index + 1..]);
}

fn hash_object(file_name: &str) {
    let content = fs::read_to_string("./".to_string() + file_name).unwrap();
    let header = format!("blob {}\0", content.len());
    let store = format!("{}{}", content, header);
    print!("{}", store);
    let mut hasher = Sha1::new();
    hasher.update(store.as_bytes());
    let res = hasher.finalize();
    let blob_sha = hex::encode(res);
    let dir_name = &blob_sha[..2];
    let file_name = &blob_sha[2..];
    fs::create_dir(format!("./.git/objects/{}", dir_name)).unwrap();
    let path = format!("./.git/objects/{}/{}", dir_name, file_name);
    fs::write(path, content).unwrap();
    print!("{}", blob_sha);
}
