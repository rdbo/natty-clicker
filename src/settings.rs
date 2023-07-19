use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Command {
    pub action: Input,
    pub listen: Input,
    pub method: Method,
    pub range: Option<CpsRange>,
}

#[derive(Deserialize, Debug)]
pub struct Input {
    pub r#type: InputType,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub enum InputType {
    Key,
    Button,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Method {
    Hold,
    Toggle,
}

#[derive(Deserialize, Debug)]
pub struct CpsRange {
    pub min: u32,
    pub max: u32,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub commands: Vec<Command>,
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let cfg = Config::builder()
            .add_source(File::with_name("Natty.toml").required(true))
            .build()?;

        cfg.try_deserialize::<Settings>()
    }
}
