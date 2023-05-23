use crate::object::{load_object, GitObject};
use anyhow::{anyhow, Result};
use std::io::{self, stdout, Cursor};

pub fn pretty_cat_file(blob_sha: &str) -> Result<()> {
    if blob_sha.len() != 40 {
        return Err(anyhow!(
            "object id should be 40 characters but was {}",
            blob_sha.len()
        ));
    }

    let GitObject {
        object_type,
        content,
    } = load_object(blob_sha)?;

    if object_type.as_str() != "blob" {
        return Err(anyhow!("Unsupported object type: {}", object_type));
    }

    io::copy(&mut Cursor::new(content), &mut stdout())?;
    Ok(())
}
