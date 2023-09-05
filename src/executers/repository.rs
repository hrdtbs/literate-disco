use crate::model::endpoint::*;
use anyhow::{Ok, Result};
use std::fs::{self, File};
use std::io::Read;
use std::process::Command;
use uuid::Uuid;

pub fn get_repository_alias(repository_name: &str) -> Result<String> {
    let alias = repository_name.split('/').last().unwrap();
    Ok(alias.to_string())
}

pub fn get_repository_ssh_path(repository_name: &String) -> Result<String> {
    Ok(format!("git@github.com:{}.git", repository_name))
}

pub fn clone_repository(ssh_path: &String) -> Result<String> {
    let cache = format!("node_modules/.endpoints-tmp/{}", Uuid::new_v4());

    Command::new("git")
        .args(["clone", "--no-checkout", "--quiet", ssh_path, &cache])
        .spawn()
        .unwrap()
        .wait()?;

    let repository_path = {
        let path_buf = fs::canonicalize(&cache)?;
        path_buf.display().to_string()
    };

    Ok(repository_path)
}

pub fn get_head_commit_hash(repository_path: &String) -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(repository_path)
        .output()?;
    let commit_hash = String::from_utf8(output.stdout)?;
    Ok(commit_hash.trim().to_string())
}

pub fn detect_main_branch(repository_path: &String) -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(repository_path)
        .output()?;
    let main_branch = String::from_utf8(output.stdout)?;
    Ok(main_branch.trim().to_string())
}

pub fn get_repository_data(
    repository_path: &String,
    branch_name: &String,
    workspace: &Option<String>,
) -> Result<EndpointSetting> {
    let target_file = {
        match workspace {
            Some(w) => format!("{}/{}/.endpoints.json", repository_path, w),
            None => format!("{}/.endpoints.json", repository_path),
        }
    };
    Command::new("git")
        .args(["checkout", branch_name, "--", &target_file])
        .current_dir(repository_path)
        .output()?;

    let mut contents = String::new();
    let mut file = File::open(target_file)?;
    file.read_to_string(&mut contents)?;

    let setting: EndpointSetting = serde_json::from_str(&contents).unwrap();
    Ok(setting)
}
