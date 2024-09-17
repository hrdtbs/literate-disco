use crate::executers::config::*;
use crate::executers::endpoint::*;
use crate::executers::repository::*;
use anyhow::{Ok, Result};

pub fn run(
    repository_name: String,
    workspace: Option<String>,
    branch: Option<String>,
) -> Result<()> {
    let mut config = read_config_file()?;

    let service = create_endpoint_files(
        repository_name.clone(),
        config.output.clone(),
        config.environment_identifier.clone(),
        workspace,
        branch,
    )?;

    config.push(get_repository_alias(&repository_name)?, service);

    write_config_file(config)?;
    Ok(())
}
