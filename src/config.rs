use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn load() -> Option<Config> {
    config_locations().into_iter().find_map(maybe_load_config)
}

#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
pub struct Config {
    pub cookie: String,
    pub intercepted: HashMap<String, Interceptor>,
}

#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
pub struct Interceptor {
    pub exe: String,

    #[serde(default = "Default::default")]
    pub args: Vec<String>,
}

fn maybe_load_config<P: AsRef<Path>>(path: P) -> Option<Config> {
    let file = fs::OpenOptions::new().read(true).open(path.as_ref()).ok()?;
    let config = ::serde_yaml::from_reader(file)
        .map_err(|serde_err| {
            eprintln!(
                "Found a config ({:?}) buf failed to parse it: {:?}",
                path.as_ref(),
                serde_err
            )
        })
        .ok()?;

    Some(config)
}

fn config_locations() -> impl IntoIterator<Item = String> {
    vec![
        "./libintercept.config".to_owned(),
        "/etc/libintercept.config".to_owned(),
    ]
}
