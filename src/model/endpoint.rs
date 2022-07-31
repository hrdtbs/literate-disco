use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct EnvList {
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

type EndpointList = HashMap<EndpointName, Endpoint>;

#[derive(Serialize, Deserialize)]
pub struct EndpointAssets {
    env: EnvList,
    pub api: EndpointList,
}

type EndpointVersion = String;

pub type EndpointSetting = HashMap<EndpointVersion, EndpointAssets>;
