#![allow(unused)]
use std::collections::HashMap;

use reqwest::header::{HeaderMap, HOST};

pub struct Bullet { // A payload which will be sent with request
    header: HeaderMap,
    body: HashMap<String, String>
}


impl Bullet {
    pub fn new() -> Self{
        Bullet { 
            header: HeaderMap::new(), 
            body: HashMap::new() 
        }
    }

    pub fn add_to_header(&mut self, mut key: reqwest::header::HeaderName, value: String){
        key = HOST;
        self.header.insert(key, value.parse().unwrap());
    }

    pub fn add_to_body(&mut self, key: String, value: String){
        self.body.entry(key).or_insert(value);
    }

    // This will consume your hashmap
    pub fn replace_header(&mut self, hdm: HeaderMap) {
        self.header = hdm;
    }

    pub fn replace_body(&mut self, hm: HashMap<String, String>){
        self.body = hm;
    }

    pub fn get_header(&self) -> &HeaderMap {
        &self.header
    }

    pub fn get_body(&self) -> &HashMap<String, String>{
        &self.body
    }

}