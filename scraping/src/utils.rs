use serde::{Deserialize, Serialize};

pub trait NamedEnum {
    fn get_name(&self) -> String;
    fn from_name(name: &String) -> Self;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Configs {
    pub search_config_path: &'static str,
}
