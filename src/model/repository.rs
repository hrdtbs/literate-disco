use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Env {
    pub local: String,
    pub dev: String,
    pub prod: String,
}
#[derive(Serialize, Deserialize)]
pub struct Endpoint {
    pub path: String,
    pub desc: String,
    pub method: Option<String>,
}

type EndpointName = String;

pub type Api = HashMap<EndpointName, Endpoint>;

#[derive(Serialize, Deserialize)]
pub struct Period {
    env: Env,
    pub api: Api,
}

type Version = String;

pub type Data = HashMap<Version, Period>;

pub struct Repository {
    pub name: String,
    pub path: String,
    pub version: String,
    pub cache: String,
    pub data: Data,
}

impl Repository {
    pub fn new(repository_name: String) -> Repository {
        Repository {
            name: repository_name.split('/').last().unwrap().to_string(),
            path: format!("git@github.com:{}.git", repository_name),
            cache: format!("node_modules/.endpoints-tmp/{}", Uuid::new_v4()),
            version: "latest".to_string(),
            data: HashMap::new(),
        }
    }
}
