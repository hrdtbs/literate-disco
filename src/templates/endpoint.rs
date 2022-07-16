use crate::model::repository::Endpoint;

// @see https://github.com/matsuri-tech/endpoints-sdk-cli/blob/dc3de607086657a0b7f33a53120804989d1c5a2a/src/templates/functions/endpoint.ts#L70
struct Param {
    name: String,
    example: Option<String>,
    param_type: String,
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

pub fn make_endpoint(name: String, e: Endpoint) {
    let pv = e.path.split('?').collect::<Vec<_>>();
    println!("{}", pv[0]);
    let qp = make_query_params(pv[1].to_string());
    println!("{}", qp[0].name);
}

#[test]
fn test_make_endpoint() {
    make_endpoint(
        "".to_string(),
        Endpoint {
            path: "/?ee".to_string(),
            desc: "".to_string(),
            method: None,
        },
    );
}
