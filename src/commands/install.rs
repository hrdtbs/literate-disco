use crate::executers::config::*;
use crate::executers::endpoint::*;
use crate::model::config::ServiceOption;
use anyhow::{Ok, Result};

pub fn run() -> Result<()> {
    let config = read_config_file()?;

    for (alias, service) in config.dependencies.iter() {
        match service.workspaces.clone() {
            Some(workspaces) => {
                for workspace in workspaces {
                    create_endpoint_files(
                        alias.clone(),
                        ServiceOption {
                            repository: service.repository.clone(),
                            branch: service.branch.clone(),
                            exclude_periods: service.exclude_periods.clone(),
                            roots: service.roots.clone(),
                        },
                        config.environment_identifier.clone(),
                        config.output.clone(),
                        Some(workspace.clone()),
                    )?;
                }
            }
            None => {
                create_endpoint_files(
                    alias.clone(),
                    ServiceOption {
                        repository: service.repository.clone(),
                        branch: service.branch.clone(),
                        exclude_periods: service.exclude_periods.clone(),
                        roots: service.roots.clone(),
                    },
                    config.environment_identifier.clone(),
                    config.output.clone(),
                    None,
                )?;
            }
        }
    }

    Ok(())
}
