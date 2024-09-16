use crate::executers::config::*;
use crate::executers::endpoint::*;
use crate::executers::repository::*;
use crate::model::config::*;
use crate::templates::endpoint::make_endpoint;
use crate::templates::root::make_root;
use crate::utils::to_camel_case::to_camel_case;
use anyhow::{Ok, Result};

pub fn run(repository_name: String, workspace: Option<String>) -> Result<()> {
    let mut config = read_config_file()?;

    let repository_alias = get_repository_alias(&repository_name)?;
    let ssh_path = get_repository_ssh_path(&repository_name)?;

    let repository_path = clone_repository(&ssh_path)?;
    let head_commit_hash = get_head_commit_hash(&repository_path)?;
    let main_branch_name = detect_main_branch(&repository_path)?;

    let repository_data = get_repository_data(&repository_path, &main_branch_name, &workspace)?;

    for (version, period) in repository_data {
        if period.env.as_ref().map_or(true, |env| env.is_empty())
            && period.api.as_ref().map_or(true, |api| api.is_empty())
        {
            continue;
        }

        let mut names: Vec<String> = Vec::new();
        let mut fns: Vec<String> = Vec::new();

        let root = make_root(
            config.environment_identifier.clone(),
            period.env.unwrap_or_default(),
        );
        fns.push(root);

        for (_name, _endpoint) in period.api.unwrap_or_default() {
            let name = to_camel_case(&_name.to_string());
            names.push(name.clone());
            fns.push(make_endpoint(name.clone(), _endpoint));
        }
        let exports = format!(
            "export const {}_{} = {{{}}};",
            to_camel_case(&repository_alias),
            to_camel_case(&version.clone()),
            names.join(",")
        );

        let filepath =
            get_endpoint_filepath(repository_alias.clone(), workspace.clone(), version.clone())?;

        write_endpoint_file(
            config.output.clone(),
            filepath,
            [fns.join("\n"), exports].join("\n"),
        )?;
    }

    config.push(
        repository_alias,
        Service {
            version: head_commit_hash,
            repository: ssh_path,
            workspaces: match workspace {
                Some(workspace) => vec![workspace],
                None => vec![],
            },
        },
    );

    write_config_file(config)?;
    Ok(())
}
