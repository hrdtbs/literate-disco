use crate::executers::config::*;
use crate::model::config::*;
use crate::model::repository::*;
use crate::templates::endpoint::make_endpoint;
use crate::utils::to_camel_case::to_camel_case;

pub fn run(repository_name: String, workspace: Option<String>) {
    let mut config = read_config_file().unwrap();
    let mut repository = Repository::new(repository_name);
    repository.clone(workspace.clone());

    for (version, period) in repository.data {
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
            version: repository.version.clone(),
            repository: repository.path.clone(),
            workspaces: match workspace {
                Some(workspace) => vec![workspace],
                None => vec![],
            },
        },
    );

    write_config_file(config).unwrap();
}
