#![allow(unused)]
use std::collections::HashMap;

use reqwest::header::{HeaderMap, HOST};

pub struct Bullet {
    // A payload which will be sent with request
    header: Option<HeaderMap>,
    body: Option<String>,
    form: Option<HashMap<String, String>>,
}

impl Bullet {
    pub fn new() -> Self {
        Bullet {
            header: None,
            form: None,
            body: None,
        }
    }

    pub fn add_to_header(&mut self, mut key: reqwest::header::HeaderName, value: String) {
        self.header.get_or_insert(HeaderMap::new()).insert(key, value.parse().unwrap());
    }

    pub fn add_to_body(&mut self, append_body: String) {
        self.body.get_or_insert(String::new()).push_str(&append_body);
    }

    pub fn add_to_form(&mut self, key: String, value: String) {
        self.form.get_or_insert(HashMap::new())
            .entry(key).or_insert(value);
    }

    pub fn build_form_from_str(&mut self, form_syn: String) {
        let mut key:String = String::new();
        let mut value: String = String::new();

        let mut flag_build_key = true;
        let mut flag_special_next = false;
        for e_char in form_syn.chars(){
            if flag_special_next {
                if flag_build_key { key.push(e_char); }
                else {value.push(e_char);}
                flag_special_next = false;
            }
            match e_char {
                '\\' => {
                    flag_special_next = true;
                },
                '\"' => {
                    if flag_build_key {

                    }
                },
                _ => {
                    if flag_build_key {
                        key.push(e_char);
                    } else {
                        value.push(e_char);
                    }
                }
            }
        }
    }

    // This will consume your hashmap
    pub fn replace_header(&mut self, new_hdm: HeaderMap) {
        match self.header {
            Some(_) => {
                self.header.replace(new_hdm);
            },
            None => {
                self.header = Some(HeaderMap::new());
            }
        }
    }

    pub fn replace_body(&mut self, new_body: String) {
        match self.body {
            Some(_) => {
                self.body.replace(new_body);
            },
            None => {
                self.body = Some(String::new());
            }
        }
    }

    pub fn get_header(&self) -> &Option<HeaderMap> {
        &self.header
    }

    pub fn get_body(&self) -> &Option<String> {
        &self.body
    }

    pub fn get_form(&self) -> &Option<HashMap<String, String>> {
        &self.form
    }



}
