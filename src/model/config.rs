use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs::File, io::Write};

pub struct ConfigOption {
    pub dependencies: HashMap<String, Service>,
    pub environment_identifier: String,
    pub output: String,
}

impl Default for ConfigOption {
    fn default() -> Self {
        Self {
            dependencies: HashMap::new(),
            environment_identifier: "process.env.NODE_ENV".to_string(),
            output: "./endpoints/".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Service {
    version: String,
    repository: String,
    workspaces: Vec<String>,
}

const CONFIG_FILE_NAME: &str = "endpoints.config.json";

pub fn write_config_file(config: Config) -> Result<()> {
    let mut file = File::open(CONFIG_FILE_NAME).expect("Config file not found");
    let c = config.publish().unwrap();
    file.write(c.as_bytes()).unwrap();
    Ok(())
}

pub fn create_config_file() -> Result<()> {
    let mut output: File = File::create(CONFIG_FILE_NAME).unwrap();
    let config = Config::new(ConfigOption {
        ..Default::default()
    })
    .publish()
    .unwrap();
    output.write_all(config.as_bytes()).unwrap();
    Ok(())
}

pub fn read_config_file() -> Result<Config> {
    let mut file = File::open(CONFIG_FILE_NAME).expect("Config file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read config file");
    let config: Config = serde_json::from_str(&contents)?;
    Ok(config)
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    dependencies: HashMap<String, Service>,
    environment_identifier: String,
    output: String,
}

impl Config {
    pub fn new(option: ConfigOption) -> Config {
        Config {
            dependencies: option.dependencies,
            environment_identifier: option.environment_identifier,
            output: option.output,
        }
    }
    // workspaceが複数のケースに対応していない
    pub fn push(&mut self, name: String, service: Service) {
        self.dependencies.insert(name, service);
    }

    pub fn publish(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
    }
}

#[test]
fn test_config_new() {
    let config = Config::new(ConfigOption {
        ..Default::default()
    });
    let json = config.publish();

    let expected = serde_json::to_string_pretty(&Config {
        dependencies: HashMap::new(),
        environment_identifier: "process.env.NODE_ENV".to_string(),
        output: "./endpoints/".to_string(),
    })
    .unwrap();

    assert_eq!(json.unwrap(), expected);
}

#[test]
fn test_config_push() {
    let mut deps = HashMap::new();
    deps.insert(
        "mes".to_string(),
        Service {
            version: "1.0.0".to_string(),
            repository: "git@github.com:matsuri-tech/endpoints-sdk-cli.git".to_string(),
            workspaces: Vec::new(),
        },
    );

    let mut config = Config::new(ConfigOption {
        dependencies: deps,
        environment_identifier: "process.env.NEXT_ENV".to_string(),
        output: "./src/endpoints/".to_string(),
    });

    config.push(
        "mes".to_string(),
        Service {
            version: "2.0.0".to_string(),
            repository: "git@github.com:matsuri-tech/endpoints-sdk-cli.git".to_string(),
            workspaces: vec!["go".to_string()],
        },
    );

    let json = config.publish().unwrap();
    let result: Config = serde_json::from_str(&json).unwrap();

    assert_eq!(
        result.dependencies["mes"].repository,
        "git@github.com:matsuri-tech/endpoints-sdk-cli.git"
    );
    assert_eq!(result.dependencies["mes"].version, "2.0.0");
    assert_eq!(result.dependencies["mes"].workspaces, vec!["go"]);
    assert_eq!(result.output, "./src/endpoints/");
    assert_eq!(result.environment_identifier, "process.env.NEXT_ENV");
}
