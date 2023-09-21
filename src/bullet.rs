#![allow(unused)]
use std::{collections::HashMap, fmt::format, str::FromStr};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, HOST};
use serde_json::Value;

#[derive(Debug)]
pub struct Bullet {
    // A payload which will be sent with request
    header: Option<HeaderMap>,
    body_raw: Option<String>,
    body_form: Option<HashMap<String, String>>,
}

impl Bullet {
    pub fn new() -> Self {
        Bullet {
            header: None,
            body_form: None,
            body_raw: None,
        }
    }

    pub fn add_to_header(&mut self, key: reqwest::header::HeaderName, value: String) {
        self.header
            .get_or_insert(HeaderMap::default())
            .insert(key, value.parse().unwrap());
    }

    pub fn add_custom_header(&mut self, key: &String, value: String) -> Result<String, String> {
        match HeaderName::from_str(key.as_str()) {
            Ok(hdn) => match HeaderValue::from_str(value.as_str()) {
                Ok(hdv) => {
                    self.header
                        .get_or_insert(HeaderMap::default())
                        .insert(hdn, hdv);
                    Ok(format!("Header {key}:{value} added."))
                }
                Err(e) => { Err(format!("{e}")) }
            },
            Err(e) => { Err(format!("{e}")) }
        }
    }

    pub fn add_to_body_raw(&mut self, append_body: String) {
        self.body_raw
            .get_or_insert(String::new())
            .push_str(&append_body);
    }

    pub fn add_to_body_form(&mut self, key: String, value: String) {
        self.body_form
            .get_or_insert(HashMap::new())
            .entry(key)
            .or_insert(value);
    }

    pub fn import_body_form(&mut self, hm: HashMap<String, String>) {
        self.body_form = Some(hm);
    }

    // This will consume your hashmap
    pub fn replace_header(&mut self, new_hdm: HeaderMap) {
        self.header.replace(new_hdm);
    }

    pub fn import_header(&mut self, new_hm: HashMap<String, String>) -> Result<String, String> {
        let mut entry_import_count = 0;
        let mut new_hdm = HeaderMap::new();
        for (k, entry) in new_hm {
            match self.add_custom_header(&k, entry) {
                Ok(res) => { entry_import_count += 1; }
                Err(e) => { return Err(format!("Failed to import header '{k}': {e}")); }
            }
        }
        Ok(format!("Imported {entry_import_count} header(s)"))
    }

    pub fn replace_body(&mut self, new_body: String) {
        self.body_raw.replace(new_body);
    }

    pub fn replace_form(&mut self, new_form: HashMap<String, String>) {
        self.body_form.replace(new_form);
    }

    pub fn get_header(&self) -> &Option<HeaderMap> {
        &self.header
    }

    pub fn get_body(&self) -> &Option<String> {
        &self.body_raw
    }

    pub fn get_form(&self) -> &Option<HashMap<String, String>> {
        &self.body_form
    }

}
