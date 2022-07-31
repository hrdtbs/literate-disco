use anyhow::{Ok, Result};
use std::{fs::File, io::Write, path::Path};

pub fn write_endpoint_file(filepath: String, contents: String) -> Result<()> {
    let mut file = File::create(filepath)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn get_endpoint_filepath(
    endpoint_dir: String,
    repository_name: String,
    workspace: Option<String>,
    version: String,
) -> Result<String> {
    let base_name = match workspace {
        Some(workspace) => [repository_name, workspace, version, "ts".to_string()].join("."),
        None => [repository_name, version, "ts".to_string()].join("."),
    };
    let file_path = Path::new(&endpoint_dir).join(base_name);
    Ok(file_path.to_str().unwrap().to_string())
}

#[test]
fn test_get_filepath() {
    let filepath = get_endpoint_filepath(
        "./endpoints/".to_string(),
        "test".to_string(),
        Some("test".to_string()),
        "test".to_string(),
    )
    .unwrap();

    assert_eq!(filepath, "./endpoints/test.test.test.ts");

    let filepath = get_endpoint_filepath(
        "./endpoints/".to_string(),
        "name".to_string(),
        None,
        "version".to_string(),
    )
    .unwrap();

    assert_eq!(filepath, "./endpoints/name.version.ts");
}
