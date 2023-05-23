use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{fs, io::Write, path::Path};

pub fn hash_object_write(path: &Path) {
    let content = fs::read_to_string(path).unwrap();
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
