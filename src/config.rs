use std::error::Error;
use std::fs;
use std::path::Path;
use toml;

pub struct Config {
    pub port: u16,
}

pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let config_path = Path::new("config/server_config.toml");
    let config_content = fs::read_to_string(config_path)?;
    let config: toml::Value = toml::from_str(&config_content)?;
    let port = match config
        .get("server")
        .and_then(|s| s.get("port"))
        .and_then(|p| p.as_integer())
    {
        Some(port) => port as u16,
        None => {
            eprintln!("Port not found in configuration; using default port 8080");
            8080
        }
    };

    Ok(Config { port })
}
