use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type EnvList = HashMap<String, String>;
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
    pub env: EnvList,
    pub api: EndpointList,
}

type EndpointVersion = String;

pub type EndpointSetting = HashMap<EndpointVersion, EndpointAssets>;
