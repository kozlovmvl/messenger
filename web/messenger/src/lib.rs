#[macro_use] extern crate diesel;
use std::env;

pub mod db;
pub mod schema;
pub mod models;
pub mod handlers;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub db_host: String
}

impl Config {
    pub fn new() -> Self {
        Self { 
            host: env::var("HOST").expect("HOST env not found."), 
            port: env::var("PORT").expect("PORT env not found.")
                .parse::<u16>().expect("Failed parse PORT to u16."),
            db_host: env::var("DB_HOST").expect("DB_HOSY env not found.")
        }
    }
}

