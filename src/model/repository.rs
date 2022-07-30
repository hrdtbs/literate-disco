use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Env {
    pub local: String,
    pub dev: String,
    pub prod: String,
}
#[derive(Serialize, Deserialize)]
pub struct Endpoint {
    pub path: String,
    pub desc: String,
    pub method: Option<String>,
}

type EndpointName = String;

pub type Api = HashMap<EndpointName, Endpoint>;

#[derive(Serialize, Deserialize)]
pub struct Period {
    env: Env,
    pub api: Api,
}

type Version = String;

pub type Data = HashMap<Version, Period>;
