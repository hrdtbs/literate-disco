use std::{collections::HashMap, process::Command};
use uuid::Uuid;

struct Env {
    local: String,
    dev: String,
    prod: String,
}

struct Endpoint {
    path: String,
    desc: String,
    method: Option<String>,
}

type EndpointName = String;

type Api = HashMap<EndpointName, Endpoint>;

struct Period {
    env: Env,
    api: Api,
}

type Version = String;

type Data = HashMap<Version, Period>;

pub struct Repository {
    pub name: String,
    pub path: String,
    pub version: String,
    cache: String,
    data: Data,
}

impl Repository {
    pub fn new(repository_name: String) -> Repository {
        Repository {
            name: repository_name.split('/').last().unwrap().to_string(),
            path: format!("git@github.com:{}.git", repository_name),
            cache: format!("./node_modules/.endpoints-tmp/{}", Uuid::new_v4()),
            version: "latest".to_string(),
            data: HashMap::new(),
        }
    }
    // versionの指定機能は使われていない気がするので後回し
    pub fn clone(&mut self, workspace: Option<String>) {
        Command::new("sh")
            .arg("-c")
            .arg(format!(
                "git clone --no-checkout --quiet {} {}",
                self.path, self.cache
            ))
            .spawn()
            .unwrap();
    }
    pub fn clean(&mut self) {
        Command::new("sh")
            .arg("-c")
            .arg(format!("rm -rf {}", self.cache))
            .spawn()
            .unwrap();
    }
}
