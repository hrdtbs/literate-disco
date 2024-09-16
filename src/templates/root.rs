use crate::model::endpoint::EnvList;

fn normalize_name(n: &str) -> &str {
    match n {
        "dev" => "development",
        "prod" => "production",
        _ => n,
    }
}

fn normalize_url(u: &str) -> String {
    if u.ends_with('/') {
        u[..u.len() - 1].to_string()
    } else {
        u.to_string()
    }
}

pub fn make_root(environment_identifier: String, env: EnvList) -> String {
    let content: String = env
        .iter()
        .map(|(n, u)| {
            format!(
                r#"
      if {} == "{}" {{
        __root = '{}';
      }}
    "#,
                environment_identifier,
                normalize_name(n),
                normalize_url(u)
            )
        })
        .collect::<Vec<String>>()
        .join("");

    format!(
        r#"
/**
 * A function that returns the URL part common to the endpoints.
 */
pub fn root() -> String {{
    let mut __root = String::new();
    {}
    __root
}}
"#,
        content
    )
}
