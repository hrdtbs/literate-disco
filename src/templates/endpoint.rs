use crate::model::repository::Endpoint;
use std::collections::HashSet;

// @see https://github.com/matsuri-tech/endpoints-sdk-cli/blob/dc3de607086657a0b7f33a53120804989d1c5a2a/src/templates/functions/endpoint.ts#L70
#[derive(PartialEq, Eq, Hash)]
struct Param {
    name: String,
    example: Option<String>,
    param_type: String,
}

fn pick_param_names(params: Vec<Param>) -> Vec<String> {
    let mut names = vec![];
    for param in params {
        names.push(param.name.clone());
    }
    names
}

fn detect_param_type(example: &str) -> String {
    match example.parse::<i32>() {
        Ok(_) => "number".to_string(),
        Err(_) => "string".to_string(),
    }
}

#[test]
fn test_detect_param_type() {
    assert_eq!(detect_param_type("333"), "number");
    assert_eq!(detect_param_type("123456789"), "number");
    assert_eq!(detect_param_type("123,456,789"), "string");
    assert_eq!(detect_param_type("20220522"), "number");
    assert_eq!(detect_param_type("2022-05-22"), "string");
    assert_eq!(detect_param_type("hello, world"), "string");
}

fn make_query_params(query_params_str: String) -> Vec<Param> {
    query_params_str
        .split('&')
        .map(|s| -> Param {
            let s = s.split('=').collect::<Vec<_>>();
            let example = if s.len() > 1 { Some(s[1]) } else { None };
            Param {
                name: s[0].to_string(),
                example: { example.map(|example| example.to_string()) },
                param_type: match example {
                    Some(example) => detect_param_type(example),
                    None => "string".to_string(),
                },
            }
        })
        .collect::<Vec<Param>>()
}

#[test]
fn test_make_query_params() {
    let params = make_query_params("location=ja&age=24&active".to_string());
    assert_eq!(params[0].name, "location");
    assert_eq!(params[0].example, Some("ja".to_string()));
    assert_eq!(params[0].param_type, "string");
    assert_eq!(params[1].name, "age");
    assert_eq!(params[1].example, Some("24".to_string()));
    assert_eq!(params[1].param_type, "number");
    assert_eq!(params[2].name, "active");
    assert_eq!(params[2].example, None);
    assert_eq!(params[2].param_type, "string");
}

fn make_path_params(path: String) -> Vec<Param> {
    path.split('/')
        .filter(|s| s.starts_with(':'))
        .map(|s| -> Param {
            Param {
                name: s[1..].to_string(),
                example: None,
                param_type: "string".to_string(),
            }
        })
        .collect::<Vec<Param>>()
}
#[test]
fn test_make_path_params() {
    let params = make_path_params("/api/v1/users/:id/items/:itemId".to_string());
    assert_eq!(params[0].name, "id");
    assert_eq!(params[0].example, None);
    assert_eq!(params[0].param_type, "string");
    assert_eq!(params[1].name, "itemId");
    assert_eq!(params[1].example, None);
    assert_eq!(params[1].param_type, "string");
}

pub fn make_endpoint(name: String, endpoint: Endpoint) {
    let pv = endpoint.path.split('?').collect::<Vec<_>>();
    let endpoint_path = pv[0].to_string();
    let query_params = {
        if pv.len() > 1 {
            make_query_params(pv[1].to_string())
        } else {
            vec![]
        }
    };
    let path_params = make_path_params(endpoint_path);

    let query_param_names = pick_param_names(query_params);
    let path_param_names = pick_param_names(path_params);
    let param_names = query_param_names
        .clone()
        .into_iter()
        .chain(path_param_names.clone().into_iter())
        .collect::<HashSet<_>>();
    println!("{:?}", query_param_names);
    println!("{:?}", path_param_names);
    println!("{:?}", param_names);
}

#[test]
fn test_make_endpoint() {
    make_endpoint(
        "".to_string(),
        Endpoint {
            path: "/:id/?ee&hoge=22&id=hoge".to_string(),
            desc: "".to_string(),
            method: None,
        },
    );
}
