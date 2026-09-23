use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub service: Vec<Service>,
}

#[derive(Debug, Deserialize)]
pub struct Service {
    pub name: String,
    pub check: CheckType,
    pub target: String, 
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckType {
    Port,
    Http,
    Process,
}

pub fn load(path: &str) -> Config {
    let contents = std::fs::read_to_string(path)
        .expect("failed to read config file");
    toml::from_str(&contents)
        .expect("failed to parse config file")
}