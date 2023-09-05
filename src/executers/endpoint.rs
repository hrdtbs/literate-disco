use anyhow::{Ok, Result};
use std::{fs::File, io::Write, path::Path};

pub fn write_endpoint_file(output: String, filename: String, contents: String) -> Result<()> {
    if !Path::new(&output).exists() {
        std::fs::create_dir_all(&output)?;
    }
    let file_path = Path::new(&output).join(filename);

    let mut file = File::create(file_path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

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
