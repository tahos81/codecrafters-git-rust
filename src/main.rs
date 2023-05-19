//use flate2;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
        println!("Initialized git directory")
    } else if args[1] == "cat-file" {
        let blob_sha = &args[3];
        let dir_name = &blob_sha[..2];
        let file_name = &blob_sha[2..];
        let path = format!("./.git/objects/{}/{}", dir_name, file_name);
        println!("{}", path);
        let data = fs::read(path).unwrap();
        println!("{:?}", data);
    } else {
        println!("unknown command: {}", args[1])
    }
}
