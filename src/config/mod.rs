#![allow(unused)]
use std::{collections::HashMap, hash};

use crate::bullet::Bullet;

mod arg_parser;

pub struct Config {
    method: reqwest::Method,
    url: String,
    gun_num: u64,
    repeat: u64,
    bullet: Bullet
}

enum HashMapType<'a> {
    StrKey(HashMap<&'a str, Vec<String>>),
    StringKey(HashMap<String, Vec<String>>),
}

impl Config {

    fn new() -> Self {
        Config {
            method: reqwest::Method::GET,
            url: String::from(""),
            gun_num: 1,
            repeat: 1,
            bullet: Bullet::new(),
        }
    }

    fn write2config(k: &str, v: Vec<String>, temp_conf: &mut Config) -> Result<String, String> {
        /*
        This function also verify that each argument key is valid to have no data */
        match k {
            "url" => match v.get(0) {
                Some(url) => {
                    temp_conf.url = url.clone();
                }
                None => {
                    return Err(String::from("URL is not specified."));
                }
            },
            "-r" | "--repeat" => match v.get(0) {
                Some(repeat_str) => {
                    let parse_res = repeat_str.parse::<u64>();
                    match parse_res {
                        Ok(r) => {
                            temp_conf.repeat = r;
                        }
                        Err(e) => {
                            return Err(format!("Argument {k} expected to be number: {e}"));
                        }
                    }
                }
                None => {
                    // Key  given
                    return Err(format!("Argument {k} (Repeat) is given but not specified."));
                }
            },
            "-n" | "--num_gun" => {
                match v.get(0) {
                    Some(num_gun_str) => {
                        let parse_res = num_gun_str.parse::<u64>();
                        match parse_res {
                            Ok(n) => {
                                temp_conf.gun_num = n;
                            }
                            Err(e) => {
                                return Err(format!("Argument {k} expected to be number: {e}"));
                            }
                        }
                    }
                    None => {
                        return Err(format!("Argument {k} (Number of concurrent process) is given but not specified."));
                    }
                }
            },
            "-b" | "--body" => { // Text body
                if !v.is_empty() {
                    temp_conf.bullet.replace_body(v.join(" "));
                } else {
                    return Err(format!("Argument {k} (text body) is given but not specified."));
                }
            },
            "-f" | "--form" => { // Text form
                // Let the bullet parse string of form
                // Syntax <key>:"<value>" or it can be <key>:<value>
            },
            "-fj" | "--form.json" => { // json format form
                // This will parse json to HashMap
            },
            "-ffj" | "--form.file.json" => {
                // This will load the content from json file and parse to hashmap
            },
            "-ft" | "--form.toml" => { // toml format form
                // parse toml format string
            },
            "-fft" | "--form.file.toml" => { // toml format form
                // load toml format string from file
            }
            _ => {
                return Err(format!("Unknown argument {k}"));
            }
        }
        Ok(String::from("Write Success"))
    }

    fn read_hashmap_conf(hm: HashMapType) -> Result<Self, Vec<String>> {
        /*
        Since there are 2 types of hashmap which are key type is str and String
        So I need to split this function
         */
        let mut temp_conf = Config::new();
        let mut err_msgs: Vec<String> = Vec::new(); // A Vector for collecting error message
        match hm {
            HashMapType::StrKey(h) => {
                for (k, v) in h {
                    match Config::write2config(k, v, &mut temp_conf) {
                        Ok(_) => { /*Do nothing keep writing config...*/ }
                        Err(e_msg) => {
                            err_msgs.push(e_msg);
                        }
                    }
                }
            }
            HashMapType::StringKey(h) => {
                for (k, v) in h {
                    match Config::write2config(k.as_str(), v, &mut temp_conf) {
                        Ok(_) => { /*Do nothing keep writing config...*/ }
                        Err(e_msg) => {
                            err_msgs.push(e_msg);
                        }
                    }
                }
            }
        }
        if err_msgs.is_empty() {
            Ok(temp_conf)
        } else {
            Err(err_msgs)
        }
    }

    // This will transform your args vector
    pub fn new_by_read_args_vec(vec_string: &mut Vec<String>) -> Result<Self, Vec<String>> {
        match arg_parser::convert2args_hashmap(vec_string) {
            Ok(h) => match Config::read_hashmap_conf(HashMapType::StrKey(h)) {
                Ok(r) => Ok(r),
                Err(e_msg) => Err(e_msg),
            },
            Err(hashmap_convert_err) => Err(vec![hashmap_convert_err]),
        }
    }

    // This will consume the argument vector!
    pub fn new_by_load_args_vec(vec_string: Vec<String>) -> Result<Self, Vec<String>> {
        match arg_parser::build_args_hashmap(vec_string) {
            Ok(h) => match Config::read_hashmap_conf(HashMapType::StringKey(h)) {
                Ok(r) => Ok(r),
                Err(e_msg) => Err(e_msg),
            },
            Err(hashmap_convert_err) => Err(vec![hashmap_convert_err]),
        }
    }

    // This will duplicate the argument vector (borrow for clone)
    pub fn new_by_import_args_vec(vec_string: &Vec<String>) -> Result<Self, Vec<String>> {
        match arg_parser::build_args_hashmap(vec_string.clone()) {
            Ok(h) => match Config::read_hashmap_conf(HashMapType::StringKey(h)) {
                Ok(r) => Ok(r),
                Err(e_msg) => Err(e_msg),
            },
            Err(hashmap_convert_err) => Err(vec![hashmap_convert_err]),
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
