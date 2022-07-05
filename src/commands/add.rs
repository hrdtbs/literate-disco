use crate::model::config::*;
use crate::model::repository::*;

pub fn run(repository_name: String, workspace: Option<String>) {
    let config = read_config_file().unwrap();
    let repository = Repository::new(repository_name);
    repository.clone(workspace);

    config.push(
        repository.name,
        Service {
            version: repository.version,
            repository: repository.path,
            workspaces: match workspace {
                Some(workspace) => vec![workspace],
                None => vec![],
            },
        },
    );
    write_config_file(config).unwrap();

    repository.clean();
}
