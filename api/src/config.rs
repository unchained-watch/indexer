use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error as IoError;
use toml;

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    dotenv: Option<ConfigTomlDotenv>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlDotenv {
    path: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub dotenv_path: String,
}

impl Config {
    pub fn new() -> Self {
        let config_filepaths: [&str; 2] = ["./Config.toml", "./config.toml"];

        let mut content: String = "".to_owned();

        for filepath in config_filepaths {
            let result: Result<String, IoError> = fs::read_to_string(filepath);

            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }

        let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Failed to create ConfigToml Object out of config file.");
            ConfigToml { dotenv: None }
        });

        let dotenv_path: String = match config_toml.dotenv {
            Some(dotenv) => dotenv.path.unwrap_or_else(|| {
                println!("Missing field path in table dotenv.");
                "unknown".to_owned()
            }),
            None => {
                println!("Missing table dotenv.");
                "unknown".to_owned()
            }
        };

        Config { dotenv_path }
    }
}
