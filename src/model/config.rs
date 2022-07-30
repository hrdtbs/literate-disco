use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub version: String,
    pub repository: String,
    pub workspaces: Vec<String>,
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
}

#[test]
fn test_config_new() {
    let config = Config::new(ConfigOption {
        ..Default::default()
    });

    let expected = serde_json::to_string_pretty(&Config {
        dependencies: HashMap::new(),
        environment_identifier: "process.env.NODE_ENV".to_string(),
        output: "./endpoints/".to_string(),
    })
    .unwrap();

    assert_eq!(serde_json::to_string_pretty(&config).unwrap(), expected);
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

    let json = serde_json::to_string_pretty(&config).unwrap();
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
