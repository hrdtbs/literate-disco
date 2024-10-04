use crate::templates::endpoint::make_endpoint;
use crate::templates::root::make_root;
use crate::utils::to_camel_case::to_camel_case;
use crate::{executers::repository::*, model::config::Service};
use anyhow::{Ok, Result};
use std::collections::HashMap;
use std::{fs::File, io::Write, path::Path};

/**
 * Return the endpoint file path
 * Patterns:
 * {repository_name}.{version}.ts
 * {repository_name}.{workspace}.{version}.ts
 * {repository_name}.{version}
 * {repository_name}.{workspace}.{version}
 */
pub fn get_endpoint_filepath(
    repository_name: String,
    workspace: Option<String>,
    version: Option<String>,
    has_extension: bool,
) -> Result<String> {
    let mut base_name = repository_name;

    if let Some(workspace) = workspace {
        base_name.push('.');
        base_name.push_str(&workspace);
    }

    if let Some(version) = version {
        base_name.push('.');
        base_name.push_str(&version);
    }

    if has_extension {
        base_name.push_str(".ts");
    }

    Ok(base_name)
}

#[test]
fn test_get_filepath() {
    let filepath = get_endpoint_filepath(
        "repository".to_string(),
        Some("workspace".to_string()),
        Some("version".to_string()),
        true,
    )
    .unwrap();

    assert_eq!(filepath, "repository.workspace.version.ts");

    let filepath =
        get_endpoint_filepath("name".to_string(), None, Some("version".to_string()), true).unwrap();

    assert_eq!(filepath, "name.version.ts");

    // no extension
    let filepath =
        get_endpoint_filepath("name".to_string(), None, Some("version".to_string()), false)
            .unwrap();
    assert_eq!(filepath, "name.version");
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

        let filepath = get_endpoint_filepath(
            repository_alias.clone(),
            workspace.clone(),
            Some(version.clone()),
            true,
        )?;

        index_imports.push(format!(
            "import * as {} from './{}';",
            to_camel_case(&version.clone()),
            get_endpoint_filepath(
                repository_alias.clone(),
                workspace.clone(),
                Some(version.clone()),
                true,
            )?,
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
        workspaces: workspace.map(|workspace| vec![workspace]),
        branch: Some(branch_name.clone()),
        exclude_periods,
        roots,
    })
}
