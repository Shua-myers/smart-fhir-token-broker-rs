use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct Config {
    pub server_port: u16,
    pub server_host: String,
    pub fhir_server_url: String,
    pub token_secret: String,
    pub log_level: String,
}

impl Config {
    pub fn load() -> Self {
        let _ = dotenvy::dotenv(); // Load from .env, but don't fail if it doesn't exist

        Config {
            server_port: env::var("SERVER_PORT")
                .unwrap_or("3000".to_string())
                .parse()
                .expect("SERVER_PORT must be a valid u16"),

            server_host: env::var("SERVER_HOST")
                .unwrap_or("0.0.0.0".to_string())
                .parse()
                .expect("SERVER_HOST must be a valid string"),

            fhir_server_url: env::var("FHIR_SERVER_URL")
                .unwrap_or("example.com".to_string())
                .parse()
                .expect("FHIR_SERVER_URL must be a valid string"),

            token_secret: env::var("TOKEN_SECRET")
                .unwrap_or("not-telling-you".to_string())
                .parse()
                .expect("TOKEN_SECRET must be a valid string"),

            log_level: env::var("LOG_LEVEL")
                .unwrap_or("debug".to_string())
                .parse()
                .expect("LOG_LEVEL must be a valid string"),
        }
    }
}
