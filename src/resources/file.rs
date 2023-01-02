use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::PathBuf,
};

use crate::error::Error;

pub struct ResFile {
    file: PathBuf,
}

impl ResFile {
    pub(crate) fn new(file: PathBuf) -> Self {
        Self { file }
    }

    /// Creates a hash value of the given resource
    pub fn hash(&self) -> Result<String, Error> {
        let mut hasher = blake3::Hasher::new();
        let file = File::open(&self.file)?;
        let buf_reader = BufReader::new(file);
        copy_wide(buf_reader, &mut hasher)?;
        let fin = hasher.finalize();
        Ok(fin.to_hex().to_string())
    }
}

// A 16 KiB buffer is enough to take advantage of all the SIMD instruction sets
// that we support, but `std::io::copy` currently uses 8 KiB. Most platforms
// can support at least 64 KiB, and there's some performance benefit to using
// bigger reads, so that's what we use here.
fn copy_wide(mut reader: impl Read, hasher: &mut blake3::Hasher) -> io::Result<u64> {
    let mut buffer = [0; 1024 * 64];
    let mut total = 0;
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => return Ok(total),
            Ok(n) => {
                hasher.update(&buffer[..n]);
                total += n as u64;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
}
