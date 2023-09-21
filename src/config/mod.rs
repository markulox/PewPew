#![allow(unused)]
use crate::bullet::Bullet;

pub mod arg_parser;
pub mod ezkey_parser;

#[derive(Debug)]
pub struct Config {
    pub method: reqwest::Method,
    pub url: String,
    pub gun_num: u64,
    pub repeat: u64,
    pub delay: Option<u64>,
    pub bullet: Bullet,
    pub verbose: bool
}

impl Config {
    pub fn new() -> Self {
        Config {
            method: reqwest::Method::GET,
            url: String::from(""),
            gun_num: 1,
            repeat: 1,
            delay: None,
            bullet: Bullet::new(),
            verbose: false
        }
    }

    pub fn change_method(&mut self, new_med: reqwest::Method) {
        self.method = new_med;
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn get_gun_num(&self) -> u64 {
        self.gun_num
    }

    pub fn get_repeat(&self) -> u64 {
        self.repeat
    }
}
