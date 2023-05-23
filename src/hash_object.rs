use anyhow::Result;
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{fs, io::Write, path::Path};

pub fn hash_object_write(path: &Path) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let header = format!("blob {}\x00", content.len());
    let store = format!("{header}{content}");

    let mut hasher = Sha1::new();
    hasher.update(store.as_bytes());
    let res = hasher.finalize();
    let blob_sha = hex::encode(res);

    let dir_name = &blob_sha[..2];
    let file_name = &blob_sha[2..];
    let mut output_path = Path::new(".git").join("objects").join(dir_name);
    if !output_path.exists() {
        fs::create_dir(output_path.clone())?;
    }
    output_path = output_path.join(file_name);

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(store.as_bytes())?;
    let encoded_store = encoder.finish()?;
    fs::write(output_path, encoded_store)?;

    print!("{blob_sha}");

    Ok(())
}
