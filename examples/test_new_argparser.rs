#![allow(unused)]
use std::collections::HashMap;

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

struct ArgParser {
    vec_args_data: Vec<(usize, Vec<String>)>,
    arg_hm: HashMap<String, usize>,
    generic_arg_data: Vec<String>,
    generic_arg_num: usize // Maximum generic argument that allow
}

impl ArgParser{
    pub fn new() -> Self {
        ArgParser { 
            vec_args_data: Vec::new(),
            arg_hm: HashMap::new(),
            generic_arg_data: Vec::new(),
            generic_arg_num: 1
        }
    }

    pub fn new_arg(&mut self, abbrv_key: &'static str, full_key: &'static str, arg_count:usize) {
        let args_data: (usize, Vec<String>) = (arg_count, Vec::new());
        self.vec_args_data.push(args_data);
        let args_data_pos = self.vec_args_data.len();
        self.arg_hm.entry(format!("-{abbrv_key}")).or_insert(args_data_pos-1);
        self.arg_hm.entry(format!("--{full_key}")).or_insert(args_data_pos-1);
    }

    pub fn new_abbrv_arg(&mut self, abbrv_key: &'static str, arg_count:usize){
        let args_data: (usize, Vec<String>) = (arg_count, Vec::new());
        self.vec_args_data.push(args_data);
        let args_data_pos = self.vec_args_data.len();
        self.arg_hm.entry(format!("-{abbrv_key}")).or_insert(args_data_pos-1);
    }

    pub fn new_full_arg(&mut self, full_key: &'static str, arg_count:usize){
        let args_data: (usize, Vec<String>) = (arg_count, Vec::new());
        self.vec_args_data.push(args_data);
        let args_data_pos = self.vec_args_data.len();
        self.arg_hm.entry(format!("--{full_key}")).or_insert(args_data_pos-1);
    }

    pub fn parse_arg_vec(&mut self, arg_vec: &mut Vec<String>) -> Result<String ,Vec<String>>{
        arg_vec.pop(); // Remove the first program arg
        let mut errors: Vec<String> = Vec::new();

        let mut curr_args_data_pos: Option<usize> = None;
        for each_arg in arg_vec {
            match check_arg_type(&each_arg){
                ArgType::IsKey => {
                    if let Some(arg_key_pos) = self.arg_hm.get(each_arg) {
                        curr_args_data_pos = Some(*arg_key_pos);
                    } else {
                        errors.push(format!("Unknown argument {each_arg}"));
                    }
                },
                ArgType::IsData => {
                    if let Some(arg_data_pos) = curr_args_data_pos {

                    } else { // This means that there is no key in front, thus it must be the generic arg?

                    }
                },
                ArgType::None => todo!(),
            }

        }
        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(String::from("Parse success"))
        }
    }

    pub fn info(&self){
        println!("[HashArgs]----");
        for (k, v) in &self.arg_hm{
            println!("{k}: pos({v})");
        }
        println!("[VecArgs]-----");
        for (s, v) in &self.vec_args_data{
            println!("size:{s} > {:?}", v);
        }

    }
}

fn main(){
    let mut incoming_arg: Vec<String> = Vec::new(); // This vec must be valid for the whole program
    for e in vec!["bin", "-key1", "v1", "-k2", "v2_1", "v2_2", "extra"] {
        incoming_arg.push(String::from(e));
    }
    println!("{:?}", incoming_arg);

    let mut arg_parser = ArgParser::new();
    arg_parser.info();
}