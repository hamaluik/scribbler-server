use std::fs::File;
use std::io;
use std::io::prelude::*;
use toml;

#[derive(Deserialize)]
pub struct JWTConfig {
    pub secret: String,
}

#[derive(Deserialize)]
pub struct HashIDSConfig {
    pub salt: String,
    pub alphabet: String,
    pub min_length: usize,
}

#[derive(Deserialize)]
pub struct Config {
    pub registration_key: String,
    pub jwt: JWTConfig,
    pub hashids: HashIDSConfig,
}

impl Config {
    pub fn load(filename: &str) -> Result<Config, io::Error> {
        let mut config_file = File::open(filename)?;
        let mut contents = String::new();
        config_file.read_to_string(&mut contents)?;
        match toml::from_str(&contents) {
            Ok(c) => Ok(c),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "couldn't parse config.toml",
            )),
        }
    }
}
