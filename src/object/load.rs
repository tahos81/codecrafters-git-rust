use crate::object::GitObject;
use anyhow::{anyhow, Result};
use flate2::bufread::ZlibDecoder;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::{Path, PathBuf},
};

pub fn load_object(blob_sha: &str) -> Result<GitObject> {
    if blob_sha.len() != 40 {
        return Err(anyhow!(
            "object id should be 40 characters but was {}",
            blob_sha.len()
        ));
    }

    let path = hash_to_path(blob_sha);
    let file = BufReader::new(File::open(path)?);
    let decoder = ZlibDecoder::new(file);
    let mut reader = BufReader::new(decoder);

    let mut buffer = Vec::new();
    reader.read_until(' ' as u8, &mut buffer)?;
    buffer.pop();

    let object_type = String::from_utf8(buffer.clone())?;

    buffer.clear();
    reader.read_until(0, &mut buffer)?;
    buffer.pop();

    let size = String::from_utf8(buffer.clone())?.parse::<usize>()?;

    let mut content = Vec::new();
    reader.read_to_end(&mut content)?;
    if content.len() != size {
        return Err(anyhow!(
            "Incorrect content length, expected {} but was {}",
            size,
            content.len()
        ));
    }

    Ok(GitObject::new(object_type, content))
}

fn hash_to_path(blob_sha: &str) -> PathBuf {
    let dir_name = &blob_sha[..2];
    let file_name = &blob_sha[2..];
    Path::new(".git")
        .join("objects")
        .join(dir_name)
        .join(file_name)
}
