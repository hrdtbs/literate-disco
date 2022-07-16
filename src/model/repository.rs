use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use std::{collections::HashMap, process::Command};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Env {
    local: String,
    dev: String,
    prod: String,
}
#[derive(Serialize, Deserialize)]
struct Endpoint {
    path: String,
    desc: String,
    method: Option<String>,
}

type EndpointName = String;

type Api = HashMap<EndpointName, Endpoint>;

#[derive(Serialize, Deserialize)]
pub struct Period {
    env: Env,
    api: Api,
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
    // versionの指定機能は使われていない気がするので後回し
    pub fn clone(&mut self, workspace: Option<String>) {
        Command::new("git")
            .args(&["clone", "--no-checkout", "--quiet", &self.path, &self.cache])
            .spawn()
            .unwrap()
            .wait()
            .expect("git clone failed");

        let cache_path = fs::canonicalize(&self.cache).unwrap().display().to_string();

        self.version = {
            let output = Command::new("git")
                .args(&["rev-parse", "HEAD"])
                .current_dir(&cache_path)
                .output()
                .expect("failed to rev-parse");
            String::from_utf8(output.stdout).unwrap().trim().to_string()
        };

        let main_branch = {
            let output = Command::new("git")
                .args(&["rev-parse", "--abbrev-ref", "origin/HEAD"])
                .current_dir(&cache_path)
                .output()
                .expect("failed to rev-parse");
            let vec = output.stdout;
            String::from_utf8(vec).unwrap().trim().to_string()
        };

        let target_file = {
            match workspace {
                Some(w) => format!("{}/{}/.endpoints.json", cache_path, w),
                None => format!("{}/.endpoints.json", cache_path),
            }
        };
        Command::new("git")
            .args(&["checkout", &main_branch, "--", &target_file])
            .current_dir(&cache_path)
            .output()
            .expect("git checkout command failed");

        let mut contents = String::new();
        File::open(target_file)
            .expect("endpoints.json not found")
            .read_to_string(&mut contents)
            .expect("failed to read endpoints.json");

        let data: Data = serde_json::from_str(&contents).unwrap();

        self.data = data;
    }
}
