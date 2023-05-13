use crate::constants;
use crate::link;
use crate::utils;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Clone)]
enum Property {
    Path,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IConfig {
    pub path: String,
    pub items: Vec<link::Link>,
}

impl IConfig {
    fn get(&self, prop: Property) -> String {
        match prop {
            Property::Path => self.path.clone(),
        }
    }
}

impl Default for IConfig {
    fn default() -> Self {
        IConfig {
            path: "".to_string(),
            items: vec![],
        }
    }
}

#[derive(Default)]
pub struct Config {
    // wrapper for IConfig
    pub default: IConfig,
    pub user: IConfig,
}

impl Config {
    fn get(&self, prop: Property) -> String {
        let v = self.user.get(prop.clone());
        match v.as_str() {
            "" => self.default.get(prop),
            _ => v,
        }
    }
    pub fn get_path(&self) -> String {
        self.get(Property::Path)
    }

    pub fn get_item(&self, target: String) -> Option<link::Link> {
        for item in &self.user.items {
            if item.target == target {
                return Some(item.to_owned());
            }
        }
        None
    }

    pub fn give_item(&mut self, link: &link::Link) {
        let ln = self.user.items.len();
        for i in 0..ln {
            if self.user.items[i].target == link.target {
                self.user.items[i] = link.clone();
                return;
            }
        }
        self.user.items.push(link.clone())
    }
}

pub fn read_iconfig(path: String) -> Result<IConfig, String> {
    let str = fs::read_to_string(path).map_err(|e| e.to_string())?;

    let result = serde_yaml::from_str(&str).map_err(|e| e.to_string())?;

    Ok(result)
}

pub fn save_iconfig(config: &IConfig, path: String) -> Result<(), String> {
    let str = serde_yaml::to_string(&config).map_err(|e| e.to_string())?;

    fs::write(utils::extend_path(&path), str).map_err(|e| e.to_string())?;

    Ok(())
}

impl Config {
    pub fn from(path: String) -> Result<Config, String> {
        let path = utils::extend_path(&path);
        let mut c = Config::default();
        let exists = Path::new(&path).exists();
        if !exists {
            c.user = IConfig::default();
        } else {
            c.user = read_iconfig(path).map_err(|e| format!("error reading config file: {}", e))?;
        }
        c.default = IConfig {
            path: constants::NIRU_FOLDER.to_owned(),
            items: vec![],
        };

        Ok(c)
    }
    pub fn new() -> Result<Config, String> {
        Config::from(constants::CONFIG_FILE.to_string())
    }
}

impl Config {
    pub fn save(&self) -> Result<(), String> {
        save_iconfig(&self.user, constants::CONFIG_FILE.to_string())
            .map_err(|e| format!("error while saving config: {}", e))?;
        Ok(())
    }
}
