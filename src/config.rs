use config::ConfigError;
use serde::*;

#[derive(Deserialize)]
pub struct Server { 
    pub host: String,
    pub port: i32
}
#[derive(Deserialize)]
pub struct Config { 
    pub server: Server 
}

impl Config { 
    pub fn from_env() -> Result<Self, ConfigError> { 
        let mut config = config::Config::new();
        config.merge(config::Environment::new())?;
        config.try_into()
    }   
}