use std::os::unix::fs::symlink;
use std::path::Path;
use std::{env, fs};

use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::utils;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct Link {
    pub source: String,
    pub target: String, // relative to niru path
}

impl Link {
    pub fn from_source(source: String) -> Result<Self, String> {
        let current_dir = match env::current_dir() {
            Ok(p) => p.to_str().unwrap().to_string(),
            Err(e) => panic!("{}", e),
        };
        let extended_source = utils::extend_path(&source);
        let source_path = Path::new(&current_dir).join(&extended_source);
        let abs_source = utils::get_absolute_path(source_path.clone())?;
        if !source_path.exists() {
            return Err(format!("source path `{abs_source}` does not exist"));
        }
        let link = Self {
            target: source,
            source: abs_source,
        };

        Ok(link)
    }
    pub fn from_target(target: String) -> Result<Self, String> {
        let config = Config::new()?;
        return match config.get_item(target.clone()) {
            None => Err(format!("couldn't find item with target `{target}`.")),
            Some(l) => Ok(l),
        };
    }

    pub fn get_absolute_target(&self) -> Result<std::path::PathBuf, String> {
        let config = Config::new()?;
        let root_path = utils::extend_path(&config.get_path());
        Ok(Path::new(&root_path).join(self.target.clone()))
    }

    fn remove_source(&self) -> Result<(), String> {
        fs::remove_file(&self.source).map_err(|e| {
            format!(
                "couldn't remove `{}`: {}",
                self.source,
                e.to_string()
            )
        })?;
        Ok(())
    }

    // store source at target path if target does not already exist
    pub fn push_source(&self) -> Result<&Self, String> {
        let absolute_target = self.get_absolute_target()?;
        if absolute_target.exists() {
            return Err(format!("target `{}` already exists.", self.target));
        }
        let abs_source = utils::get_absolute_path(absolute_target)?;
        fs::copy(self.source.clone(), &abs_source).map_err(|e| {
            format!(
                "couldn't move `{}` to `{}`: {}",
                self.source,
                abs_source,
                e.to_string()
            )
        })?;

        return Ok(self);
    }

    // setup link
    pub fn pull_target(&self) -> Result<&Self, String> {
        let absolute_target = self.get_absolute_target()?;
        if !absolute_target.exists() {
            return Err(format!("target `{}` doesn't exist.", self.target));
        }
        let absolute_source = utils::extend_path(&self.source);
        if Path::new(&absolute_source).exists() {
            // remove source
            self.remove_source()?;
        }
        let abs_source = utils::get_absolute_path(absolute_target)?;
        symlink(&abs_source, &absolute_source).map_err(|e| {
            format!(
                "couldn't link `{}` to `{}`: {}",
                abs_source,
                absolute_source,
                e.to_string()
            )
        })?;

        return Ok(self);
    }

    // TODO
    pub fn register(&self) -> Result<(), String> {
        let mut config = Config::new()?;
        config.give_item(self);
        config.save()?;
        Ok(())
    }
}
