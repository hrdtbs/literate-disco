use crate::model::endpoint::EnvList;

fn normalize_name(n: &str) -> &str {
    match n {
        "dev" => "development",
        "prod" => "production",
        _ => n,
    }
}

fn normalize_url(u: &str) -> String {
    if let Some(stripped) = u.strip_suffix('/') {
        stripped.to_string()
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
    if ({} == "{}") {{
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
export const root = () => {{
    let __root = "";
    {}
    return __root
}}
"#,
        content
    )
}
