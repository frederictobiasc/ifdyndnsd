use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::net::{IpAddr, Ipv6Addr};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TsigKey {
    pub server: IpAddr,
    pub name: String,
    pub alg: String,
    pub secret: String,
    // TODO:
    // #[serde(rename = "secret-base64")]
    // pub secret_base64: Option<String>,
    // #[serde(rename = "secret-file")]
    // pub secret_file: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Interface {
    pub key: String,
    pub name: String,
    pub interface: String,
    pub scope: Option<String>,
    pub neighbors: Option<HashMap<String, Ipv6Addr>>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub keys: HashMap<String, TsigKey>,
    pub a: Vec<Interface>,
    pub aaaa: Vec<Interface>,
}

pub fn load(filename: &str) -> Result<Config, String> {
    let mut f = File::open(filename)
        .map_err(|e| format!("{}", e))?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)
        .map_err(|e| format!("{}", e))?;
    let config = toml::from_str(&buf)
        .map_err(|e| format!("{}", e))?;
    Ok(config)
}