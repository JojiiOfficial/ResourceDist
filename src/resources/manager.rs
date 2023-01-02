use super::Resource;
use crate::config::{self, Config, Directory};

/// Helper for managing resources
pub struct ResManager<'a> {
    dirs: &'a config::Resources,
}

impl<'a> ResManager<'a> {
    pub fn new(dirs: &'a config::Resources) -> Self {
        Self { dirs }
    }

    #[inline]
    pub fn from_cfg(config: &'a Config) -> Self {
        Self::new(&config.resources)
    }

    /// Returns the directory config for the given name
    pub fn get_dir(&self, name: &str) -> Option<&Directory> {
        self.dirs.directories.iter().find(|i| i.name == name)
    }

    /// Gets a resource by its name
    pub fn get(&'a self, name: &str) -> Option<Resource<'a>> {
        Some(Resource::new(self.get_dir(name)?))
    }
}

/// Returns the resource manager with the currently loaded configuration
#[inline]
pub fn get() -> ResManager<'static> {
    ResManager::from_cfg(crate::config())
}
