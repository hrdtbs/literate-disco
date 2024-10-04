use crate::executers::config::*;
use crate::executers::endpoint::*;
use crate::executers::repository::*;
use anyhow::{Ok, Result};

pub fn run(
    repository_name: String,
    workspace: Option<String>,
    branch: Option<String>,
    exclude_periods: Option<Vec<String>>,
) -> Result<()> {
    let mut config = read_config_file()?;

    let alias = get_repository_alias(&repository_name)?;

    let service = create_endpoint_files(
        repository_name.clone(),
        config.output.clone(),
        config.environment_identifier.clone(),
        workspace,
        branch,
        exclude_periods,
        config
            .dependencies
            .get(&alias)
            .and_then(|service| service.roots.clone()),
    )?;

    config.push(alias, service);

    write_config_file(config)?;
    Ok(())
}
