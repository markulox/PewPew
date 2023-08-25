#![allow(unused)]

use std::{collections::HashMap, vec};

enum ArgType {
    IsKey,
    IsData,
    None,
}

fn check_arg_type(check_arg: &str) -> ArgType {
    match check_arg.chars().next() {
        None => ArgType::None,
        Some(ch) => {
            if ch == '-' {
                ArgType::IsKey
            } else {
                ArgType::IsData
            }
        }
    }
}

// Let this function consume the vector of argument
pub fn build_args_hashmap(
    mut vec_args: Vec<String>,
) -> Result<HashMap<String, Vec<String>>, String> {
    let mut hashmap: HashMap<String, Vec<String>> = HashMap::new();
    vec_args.remove(0); // Remove program name parameters
    match vec_args.pop() {
        // Check the last argument
        Some(v) => {
            // Last element exist
            match check_arg_type(&v) {
                ArgType::IsData => {
                    // If it is data, assume it as url
                    hashmap.insert(String::from("url"), vec![v]);
                }
                _ => {
                    return Err(String::from("Expected last argument to be URL."));
                }
            }
        }
        None => {
            return Err(String::from("Nothing specified."));
        }
    }

    // Prepare curr_key for ชี้โบ๊ชี้เบ๊
    let mut curr_key: Option<&String> = None;
    for each_arg in &vec_args {
        match check_arg_type(each_arg) {
            ArgType::IsKey => {
                curr_key = Some(each_arg);
            }
            ArgType::IsData => {
                if let Some(curr_key) = curr_key {
                    hashmap
                        .entry(String::from(curr_key))
                        .and_modify(|str_vec| {
                            str_vec.push(String::from(each_arg));
                        })
                        .or_insert(vec![String::from(each_arg)]);
                } else {
                    //if curr_key still none, thats mean there is no key given
                    return Err(String::from("Unexpected arguments."));
                }
            }
            ArgType::None => {
                return Err(String::from("Unknown data type"));
            }
        }
    }

    Ok(hashmap)
}

// Let this function borrow the argument string
pub fn convert_args2hashmap(
    vec_args: &'_ mut Vec<String>,
) -> Result<HashMap<&'_ str, Vec<String>>, String> {
    let mut hashmap: HashMap<&str, Vec<String>> = HashMap::new();

    // Remove the first arg (program name) from the argument string
    vec_args.remove(0);
    // Now check the last element that exist?
    match vec_args.pop() {
        Some(v) => {
            // If there is something, let's check its a key or data
            match check_arg_type(&v) {
                // If it is data, it might be a url?
                // next version we can implement url_verfication()
                ArgType::IsData => {
                    hashmap.insert("url", vec![v]);
                }
                _ => {
                    return Err(String::from("Expected last argument to be URL."));
                }
            }
        }
        None => {
            // If not, thats mean user didn't input anything
            return Err(String::from("Nothing specified."));
        }
    }

    // Now start to collect key and value
    let mut curr_key: Option<&str> = None;
    for i in 0..vec_args.len() {
        // We need to loop this way, because if we use for each the string data will be moved to the inner scope
        // which is the owner will be killed after scope end
        match check_arg_type(&vec_args[i]) {
            ArgType::IsKey => {
                curr_key = Some(&vec_args[i]);
                hashmap.entry(&vec_args[i]).or_default();
            }
            ArgType::IsData => match curr_key {
                Some(k) => {
                    hashmap.entry(k).and_modify(|curr_dat| {
                        curr_dat.push(String::from(&vec_args[i]));
                    });
                }
                None => {
                    return Err(String::from("Unexpected arguments."));
                }
            },
            ArgType::None => {}
        }
    }
    Ok(hashmap)
}
