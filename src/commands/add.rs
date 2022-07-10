use crate::model::config::*;
use crate::model::repository::*;

pub fn run(repository_name: String, workspace: Option<String>) {
    let mut config = read_config_file().unwrap();
    let mut repository = Repository::new(repository_name);
    repository.clone(workspace.clone());

    config.push(
        repository.name.clone(),
        Service {
            version: repository.version.clone(),
            repository: repository.path.clone(),
            workspaces: match workspace {
                Some(workspace) => vec![workspace],
                None => vec![],
            },
        },
    );
    write_config_file(config).unwrap();

    repository.clean();
}
