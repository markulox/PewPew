#![allow(unused)]
use std::{collections::HashMap, hash};

mod arg_parser;

pub struct Config {
    url: String,
    gun_num: u64,
    repeat: u64,
}

enum HashMapType<'a> {
    StrKey(HashMap<&'a str, Vec<String>>),
    StringKey(HashMap<String, Vec<String>>),
}

impl Config {
    pub fn new() -> Self {
        Config {
            url: String::from(""),
            gun_num: 0,
            repeat: 0,
        }
    }

    fn write_config(k: &str, v: Vec<String>, temp_conf: &mut Config) -> Result<String, String> {
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
                            return Err(String::from("Argument {k} expected to be number: {e}"));
                        }
                    }
                }
                None => {
                    return Err(String::from("{k} (Repeat) is not specified."));
                }
            },
            "-n" | "--num_gun" => {
                let parse_res = v[0].parse::<u64>();
                match parse_res {
                    Ok(n) => {
                        temp_conf.gun_num = n;
                    }
                    Err(e) => {
                        return Err(String::from("Argument {k} expected to be number: {e}"));
                    }
                }
            }
            _ => {
                return Err(String::from("Unknown argument {k}"));
            }
        }
        Ok(String::from("Write Success"))
    }

    fn read_conf(hm: HashMapType) -> Result<Self, String> {
        /*
        Since there are 2 types of hashmap which are key type is str and String
        So I need to split this function
         */
        let mut temp_conf = Config::new();
        match hm {
            HashMapType::StrKey(h) => {
                for (k, v) in h {
                    match Config::write_config(k, v, &mut temp_conf) {
                        Ok(_) => { /*Do nothing keep writing config...*/ }
                        Err(e_msg) => {
                            return Err(e_msg);
                        }
                    }
                }
            }
            HashMapType::StringKey(h) => {
                for (k, v) in h {
                    match Config::write_config(k.as_str(), v, &mut temp_conf) {
                        Ok(_) => { /*Do nothing keep writing config...*/ }
                        Err(e_msg) => {
                            return Err(e_msg);
                        }
                    }
                }
            }
        }
        Ok(temp_conf)
    }

    // This will transform your args vector
    pub fn new_by_read_args_vec(vec_string: &mut Vec<String>) -> Result<Self, String> {
        match arg_parser::convert_args2hashmap(vec_string) {
            Ok(h) => match Config::read_conf(HashMapType::StrKey(h)) {
                Ok(r) => Ok(r),
                Err(e_msg) => Err(e_msg),
            },
            Err(hashmap_convert_err) => Err(hashmap_convert_err),
        }
    }

    // This will consume the argument vector!
    pub fn new_by_load_args_vec(vec_string: Vec<String>) -> Result<Self, String> {
        match arg_parser::build_args_hashmap(vec_string) {
            Ok(h) => match Config::read_conf(HashMapType::StringKey(h)) {
                Ok(r) => Ok(r),
                Err(e_msg) => Err(e_msg),
            },
            Err(hashmap_convert_err) => Err(hashmap_convert_err),
        }
    }

    // This will duplicate the argument vector
    pub fn new_by_import_args_vec(vec_string: &Vec<String>) -> Result<Self, String> {
        match arg_parser::build_args_hashmap(vec_string.clone()) {
            Ok(h) => match Config::read_conf(HashMapType::StringKey(h)) {
                Ok(r) => Ok(r),
                Err(e_msg) => Err(e_msg),
            },
            Err(hashmap_convert_err) => Err(hashmap_convert_err),
        }
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn get_gun_num(&self) -> &u64 {
        &self.gun_num
    }

    pub fn get_repeat(&self) -> &u64 {
        &self.repeat
    }
}
