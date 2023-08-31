use std::{collections::HashMap, fmt::format};

fn build_form_from_str(form_syn: String) -> Result<HashMap<String, String>, String> {
    let mut hm = HashMap::new();

    #[derive(Debug)]
    enum CollectState {
        PrepKey, BuildKey, PrepVal, BuildVal
    }

    #[derive(Debug)]
    enum StrParseState {
        EndKey, Default, SpecialChar, InStr, OutStr, InStrSpecial
    }

    let mut key: String = String::new();
    let mut value: String = String::new();

    let mut mainstate = CollectState::PrepKey;
    let mut substate: StrParseState = StrParseState::Default;

    for e_char in form_syn.chars() {
        // Here will be state management
        // we will check sub state first (str_parse_state)
        print!("CHAR[{}] ",e_char);
        match substate {
            StrParseState::Default => {
                match mainstate {
                    CollectState::PrepKey => {
                        if key.len() > 0 { // If there are any char in string, flush to hm
                            hm.insert(key.clone(), value.clone());
                            key = String::new();
                            value = String::new();
                        }
                        match e_char {
                            ' ' => { continue; },
                            ':' => { return Err(String::from("System Crash (No Key given)")); },
                            '\\' => {
                                mainstate = CollectState::BuildKey;
                                substate = StrParseState::SpecialChar;
                            }
                            '\"' => {
                                key.push(e_char);
                                mainstate = CollectState::BuildKey;
                                substate = StrParseState::InStr;
                            },
                            _ => {
                                key.push(e_char);
                                mainstate = CollectState::BuildKey;
                                substate = StrParseState::Default;
                            }
                        }
                    },
                    CollectState::BuildKey => {
                        match e_char {
                            ' ' => { substate = StrParseState::EndKey; },
                            '\"' => { return Err(String::from("System Crash (Invalid Key Format)")); /* System crash (Invalid Key Format) */}
                            '\\' => { substate = StrParseState::SpecialChar; },
                            ':' => { mainstate = CollectState::PrepVal;}
                            _ => { key.push(e_char); }
                        }
                    },
                    CollectState::PrepVal => {
                        match e_char {
                            ' ' => { continue; },
                            ':' => { return Err(String::from("System Crash (Invalid Value Format)"));/* System crash (Invalid Value format) */ },
                            '\\' => {
                                mainstate = CollectState::BuildVal;
                                substate = StrParseState::SpecialChar;
                            }
                            '\"' => {
                                value.push(e_char);
                                mainstate = CollectState::BuildVal;
                                substate = StrParseState::InStr;
                            }
                            _ => {
                                value.push(e_char);
                                mainstate = CollectState::BuildVal;
                                substate = StrParseState::Default;
                            }
                        }
                    },
                    CollectState::BuildVal => {
                        match e_char {
                            '\"' | ':' => {return Err(String::from("System Crash (Invalid Value Format)")); /* System Crash (Invalid Value Format) */ },
                            ' ' => { mainstate = CollectState::PrepKey; }
                            '\\' => { substate = StrParseState::SpecialChar; }
                            _ => { value.push(e_char); }
                        }
                    },
                }
            },
            StrParseState::EndKey => {
                match mainstate {
                    CollectState::BuildKey => {
                        match e_char {
                            ' ' => { continue; },
                            ':' => {
                                mainstate = CollectState::PrepVal;
                                substate = StrParseState::Default;
                            },
                            _ => { return Err(String::from("System Crash (Invalid Key Format)"));/* System Crash (Invalid Key Format) */ }
                        }
                    },
                    _ => { return Err(String::from("System Crash (Out of working state)")); /* System Crash (Out of working state) */ }
                }
            },
            StrParseState::SpecialChar => {
                match mainstate {
                    CollectState::BuildKey => { key.push(e_char); },
                    CollectState::BuildVal => { value.push(e_char); },
                    _ => { return Err(String::from("System Crash (Out of working state)")); /* System Crash (Out of working state) */ }
                }
                substate = StrParseState::Default;
            },
            StrParseState::InStr => {
                match mainstate {
                    CollectState::BuildKey => { 
                        match e_char{
                            '\"' => {
                                key.push(e_char);
                                substate = StrParseState::OutStr;
                            },
                            '\\' => {
                                substate = StrParseState::InStrSpecial;
                            },
                            _ => { key.push(e_char); }
                        }
                    },
                    CollectState::BuildVal => { 
                        match e_char{
                            '\"' => {
                                value.push(e_char);
                                mainstate = CollectState::PrepKey;
                                substate = StrParseState::Default;
                            },
                            '\\' => {
                                substate = StrParseState::InStrSpecial;
                            },
                            _ => { value.push(e_char); }
                        }
                    },
                    _ => { return Err(String::from("System Crash (Out of working state)"));/* System Crash (Out of working state) */ }
                }
            },
            StrParseState::OutStr => {
                match mainstate {
                    CollectState::BuildKey => {
                        match e_char {
                            ' ' => {continue;}
                            ':' => {
                                mainstate = CollectState::PrepVal;
                                substate = StrParseState::Default;
                            },
                            _ => { return Err(String::from("System Crash (Invalid Key Format)"));/* System Crash (Invalid Key Format) */ }
                        }
                    },
                    _ => { return Err(String::from("System Crash (Out of working state)"));/* System Crash (Out of working state) */ }
                }
            }
            StrParseState::InStrSpecial => {
                match mainstate {
                    CollectState::BuildKey => { key.push(e_char); },
                    CollectState::BuildVal => { value.push(e_char); },
                    _ => { return Err(String::from("System Crash (Out of working state)"));/* System Crash (Out of working state) */ }
                }
                substate = StrParseState::InStr;
            },
        }
        print!(" M:{:?} S:{:?}\n", mainstate, substate);
    }
    // Any remaining string key and string value shall be added to hashmap
    if key.len() > 0 && value.len() > 0 { hm.insert(key.clone(), value.clone()); }
    else { return Err(String::from("System Crash (Invalid Value Syntax)"));/* System Crash */}
    return Ok(hm);
}

fn main() {
    let test_form_syn = "key1:pbdr \"key2\":\"LDVR 2.0\" \\:NEWKEY3:\"Test\\\\\"Aphost\"";
    let form_syn = String::from(test_form_syn);
    let parse_res = build_form_from_str(form_syn);
    match parse_res {
        Ok(hm) => {
            println!("Hashmap Value");
            for (k,v) in hm {
                println!("K[{}] => {}", k, v);
            }
        },
        Err(e) => {
            println!("<X> Parse error: {e}");
        },
    }

}
