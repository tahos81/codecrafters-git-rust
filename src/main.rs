use flate2::read::ZlibDecoder;
use std::env;
use std::fs;
use std::io::Read;

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
        let data = fs::read(path).unwrap();
        let mut d = ZlibDecoder::new(data.as_slice());
        let mut result = String::new();
        d.read_to_string(&mut result).unwrap();
        let header_index = result.find('\0').unwrap();
        print!("{}", &result[header_index + 1..]);
    } else {
        println!("unknown command: {}", args[1])
    }
}
