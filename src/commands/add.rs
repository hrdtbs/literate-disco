use crate::executers::config::*;
use crate::executers::repository::*;
use crate::model::config::*;
use crate::model::repository::*;
use crate::templates::endpoint::make_endpoint;
use crate::utils::to_camel_case::to_camel_case;
use anyhow::{Ok, Result};

pub fn run(repository_name: String, workspace: Option<String>) -> Result<()> {
    let mut config = read_config_file()?;
    let repository = Repository::new(repository_name);

    let repository_path = clone_repository(&repository)?;
    let head_commit_hash = get_head_commit_hash(&repository_path)?;
    let main_branch_name = detect_main_branch(&repository_path)?;

    let repository_data = get_repository_data(&repository_path, &main_branch_name, &workspace)?;

    for (version, period) in repository_data {
        let mut names: Vec<String> = Vec::new();
        let mut fns: Vec<String> = Vec::new();

        for (_name, _endpoint) in period.api {
            let name = to_camel_case(&_name.to_string());
            names.push(name.clone());
            fns.push(make_endpoint(name.clone(), _endpoint));
        }
        let exports = format!(
            "export const {}_{} = {{{}}};",
            to_camel_case(&repository.name.clone()),
            to_camel_case(&version.clone()),
            names.join(",")
        );
        println!("{}", exports);
    }

    config.push(
        repository.name.clone(),
        Service {
            version: head_commit_hash,
            repository: repository.path,
            workspaces: match workspace {
                Some(workspace) => vec![workspace],
                None => vec![],
            },
        },
    );

    write_config_file(config)?;
    Ok(())
}
