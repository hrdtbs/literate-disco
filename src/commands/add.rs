use crate::executers::config::*;
use crate::executers::endpoint::*;
use crate::executers::repository::*;
use crate::model::config::ServiceOption;
use anyhow::{Ok, Result};

pub fn run(
    repository_name: String,
    workspace: Option<String>,
    branch: Option<String>,
    exclude_periods: Option<Vec<String>>,
) -> Result<()> {
    let mut config = read_config_file()?;

    let alias = get_repository_alias(&repository_name)?;
    let repository_path = get_repository_path(&repository_name)?;

    let service = create_endpoint_files(
        alias.clone(),
        ServiceOption {
            repository: repository_path,
            branch,
            exclude_periods,
            roots: None,
        },
        config.environment_identifier.clone(),
        config.output.clone(),
        workspace,
    )?;

    config.push(alias, service);

    write_config_file(config)?;
    Ok(())
}
