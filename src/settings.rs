use config::{Config, ConfigError, Environment, File};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Cluster {
    pub name: String,
}

impl Default for Cluster {
    fn default() -> Self {
        Self {
            name: "smol".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    pub cluster: Cluster,
}
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_path = match home_dir() {
            Some(path) => path.join(".smolk8s/config.yaml"),
            None => PathBuf::from(".smolk8s/config.yaml"),
        };

        if !config_path.exists() {
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent).map_err(|e| ConfigError::Message(e.to_string()))?;
            }

            let default_settings = Settings::default();
            let yaml = serde_yaml::to_string(&default_settings)
                .map_err(|e| ConfigError::Message(e.to_string()))?;

            fs::write(&config_path, yaml).map_err(|e| ConfigError::Message(e.to_string()))?;

            println!("default configuration created at {:?}", config_path);
        }

        let path = match Some(config_path.to_str()) {
            Some(path) => path,
            None => {
                println!("no path found");
                None
            }
        };

        let settings = Config::builder()
            .add_source(File::with_name(path.expect("no path found")))
            .build()
            .unwrap();

        settings.try_deserialize()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            cluster: Cluster::default(),
        }
    }
}

// use serde_yaml;
// use std::fs;
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Config {
//     path: PathBuf,
//     cluster: Cluster,
// }
//
// impl Config {
//     pub fn new() -> Self {
//         Self {
//             path: path,
//             cluster: Cluster::default(),
//         }
//     }
//
//     pub fn load() -> Result<String, serde_yaml::Error> {
//         let path = home_dir().unwrap().join(".smolk8s/config.yaml");
//         let file = fs::File::open(path).unwrap();
//         let content: String = serde_yaml::from_reader(&file)?;
//         let config: String = serde_yaml::from_str(&content)?;
//         Ok(config)
//     }
// }
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Settings::new();
        assert_eq!(config.unwrap().cluster.name, "smol".to_string());
    }
}
// "/Users/chrisaddy/.smolk8s/config.yaml"
