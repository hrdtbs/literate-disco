use crate::templates::endpoint::make_endpoint;
use crate::templates::root::make_root;
use crate::utils::to_camel_case::to_camel_case;
use crate::{executers::repository::*, model::config::Service};
use anyhow::{Ok, Result};
use std::collections::HashMap;
use std::{fs::File, io::Write, path::Path};

pub fn get_endpoint_filepath(
    repository_name: String,
    workspace: Option<String>,
    version: String,
) -> Result<String> {
    let base_name = match workspace {
        Some(workspace) => [repository_name, workspace, version, "ts".to_string()].join("."),
        None => [repository_name, version, "ts".to_string()].join("."),
    };
    Ok(base_name)
}

#[test]
fn test_get_filepath() {
    let filepath = get_endpoint_filepath(
        "test".to_string(),
        Some("test".to_string()),
        "test".to_string(),
    )
    .unwrap();

    assert_eq!(filepath, "test.test.test.ts");

    let filepath = get_endpoint_filepath("name".to_string(), None, "version".to_string()).unwrap();

    assert_eq!(filepath, "name.version.ts");
}

pub fn write_endpoint_file(output: String, filename: String, contents: String) -> Result<()> {
    if !Path::new(&output).exists() {
        std::fs::create_dir_all(&output)?;
    }
    let file_path = Path::new(&output).join(filename);

    let mut file = File::create(file_path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn create_endpoint_files(
    repository_name: String,
    output: String,
    environment_identifier: String,
    workspace: Option<String>,
    branch: Option<String>,
    exclude_periods: Option<Vec<String>>,
    roots: Option<HashMap<String, String>>,
) -> Result<Service> {
    let repository_alias = get_repository_alias(&repository_name)?;
    let ssh_path = get_repository_ssh_path(&repository_name)?;
    let repository_path = clone_repository(&ssh_path)?;
    let head_commit_hash = get_head_commit_hash(&repository_path)?;
    let branch_name = match branch {
        Some(branch) => branch,
        None => detect_main_branch(&repository_path)?,
    };
    let repository_data = get_repository_data(&repository_path, &branch_name, &workspace)?;

    let mut index_imports: Vec<String> = Vec::new();
    let mut index_exports_names: Vec<String> = Vec::new();

    for (version, period) in repository_data {
        if exclude_periods
            .as_ref()
            .map_or(false, |excludes| excludes.contains(&version))
        {
            continue;
        }

        if period.env.as_ref().map_or(true, |env| env.is_empty())
            && period.api.as_ref().map_or(true, |api| api.is_empty())
        {
            continue;
        }

        let mut names: Vec<String> = Vec::new();
        let mut fns: Vec<String> = Vec::new();

        let root = make_root(
            environment_identifier.clone(),
            period.env.unwrap_or_default(),
            roots.clone(),
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

        index_imports.push(format!(
            "import * as {} from './{}.{}';",
            to_camel_case(&version.clone()),
            &repository_alias,
            version.clone()
        ));
        index_exports_names.push(to_camel_case(&version.clone()));

        write_endpoint_file(
            output.clone(),
            filepath,
            [fns.join("\n"), exports].join("\n"),
        )?;
    }

    write_endpoint_file(
        output.clone(),
        [repository_alias.clone(), "ts".to_string()].join("."),
        [
            index_imports.join("\n"),
            format!(
                "export const {} = {{{}}};",
                to_camel_case(&repository_alias),
                index_exports_names.join(",")
            ),
        ]
        .join("\n"),
    )?;

    Ok(Service {
        version: head_commit_hash,
        repository: ssh_path,
        workspaces: match workspace {
            Some(workspace) => vec![workspace],
            None => vec![],
        },
        branch: Some(branch_name.clone()),
        exclude_periods: None,
        roots: None,
    })
}
