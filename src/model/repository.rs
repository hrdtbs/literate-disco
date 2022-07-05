use std::collections::HashMap;

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
            cache: format!("./node_modules/.endpoints-tmp/{}", repository_name),
            version: "latest".to_string(),
            data: HashMap::new(),
        }
    }
    // versionの指定機能は使われていない気がするので後回し
    pub fn clone(&mut self, workspace: Option<String>) {
        todo!("clone repository");
    }
    pub fn clean(&mut self) {
        todo!("clean repository");
    }
}
