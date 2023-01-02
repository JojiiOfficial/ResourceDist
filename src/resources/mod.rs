pub mod file;
pub mod manager;

use self::file::ResFile;
use crate::{config::Directory, error::Error};
use std::path::{Path, PathBuf};

pub use manager::get;

/// A single configured resource
pub struct Resource<'a> {
    dir: &'a Directory,
}

impl<'a> Resource<'a> {
    pub fn new(dir: &'a Directory) -> Self {
        Self { dir }
    }

    pub fn get_file(&self, name: &str) -> Result<ResFile, Error> {
        let path = self.get_escaped_filepath(name)?;
        if !path.exists() {
            return Err(Error::NotFound);
        }

        Ok(ResFile::new(path))
    }

    /// Strips a potentially passed directory path from the filepath and joins this to the current resource
    /// path. This prevents filepath injection
    fn get_escaped_filepath(&self, filepath: &str) -> Result<PathBuf, Error> {
        let name_path = Path::new(filepath)
            .file_name()
            .and_then(|i| i.to_str())
            .ok_or(Error::Internal)?;
        Ok(self.get_path().join(name_path))
    }

    pub fn get_path(&self) -> &Path {
        Path::new(&self.dir.path)
    }
}
