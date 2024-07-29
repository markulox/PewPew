/* This module will attemp to parse string the in the format of simple key and value which is
<key>:<value> or
"<key>":"<value>"
I will named this format to "EZKey"
*/

use std::collections::HashMap;

enum CollectState {
    PrepKey,
    BuildKey,
    PrepVal,
    BuildVal,
}

enum StrParseState {
    EndKey,
    Default,
    SpecialChar,
    InStr,
    OutStr,
    InStrSpecial,
}

pub struct ParseErr {}
impl ParseErr {
    const NO_KEY: &'static str = "No Key given";
    const INVL_KEY_FMT: &'static str = "Invalid Key Format";
    const INVL_VAL_FMT: &'static str = "Invalid Value Format";
    const OUT_STATE: &'static str = "Out of expected state";
}

pub fn parse_to_hashmap(form_syn: String) -> Result<HashMap<String, String>, &'static str> {
    let mut hm = HashMap::new();

    let mut key: String = String::new();
    let mut value: String = String::new();

    let mut mainstate = CollectState::PrepKey;
    let mut substate: StrParseState = StrParseState::Default;

    for e_char in form_syn.chars() {
        // Here will be state management
        // we will check sub state first (str_parse_state)
        match substate {
            StrParseState::Default => {
                match mainstate {
                    CollectState::PrepKey => {
                        if key.len() > 0 {
                            // If there are any char in string, flush to hm
                            hm.insert(key.clone(), value.clone());
                            key = String::new();
                            value = String::new();
                        }
                        match e_char {
                            ' ' => {
                                continue;
                            }
                            ':' => {
                                return Err(ParseErr::NO_KEY);
                            }
                            '\\' => {
                                mainstate = CollectState::BuildKey;
                                substate = StrParseState::SpecialChar;
                            }
                            '\"' => {
                                key.push(e_char);
                                mainstate = CollectState::BuildKey;
                                substate = StrParseState::InStr;
                            }
                            _ => {
                                key.push(e_char);
                                mainstate = CollectState::BuildKey;
                                substate = StrParseState::Default;
                            }
                        }
                    }
                    CollectState::BuildKey => match e_char {
                        ' ' => {
                            substate = StrParseState::EndKey;
                        }
                        '\"' => {
                            return Err(ParseErr::INVL_KEY_FMT);
                        }
                        '\\' => {
                            substate = StrParseState::SpecialChar;
                        }
                        ':' => {
                            mainstate = CollectState::PrepVal;
                        }
                        _ => {
                            key.push(e_char);
                        }
                    },
                    CollectState::PrepVal => match e_char {
                        ' ' => {
                            continue;
                        }
                        ':' => {
                            return Err(ParseErr::INVL_VAL_FMT);
                        }
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
                    },
                    CollectState::BuildVal => match e_char {
                        '\"' | ':' => {
                            return Err(ParseErr::INVL_VAL_FMT);
                        }
                        ' ' => {
                            mainstate = CollectState::PrepKey;
                        }
                        '\\' => {
                            substate = StrParseState::SpecialChar;
                        }
                        _ => {
                            value.push(e_char);
                        }
                    },
                }
            }
            StrParseState::EndKey => match mainstate {
                CollectState::BuildKey => match e_char {
                    ' ' => {
                        continue;
                    }
                    ':' => {
                        mainstate = CollectState::PrepVal;
                        substate = StrParseState::Default;
                    }
                    _ => {
                        return Err(ParseErr::INVL_KEY_FMT);
                    }
                },
                _ => {
                    return Err(ParseErr::OUT_STATE);
                }
            },
            StrParseState::SpecialChar => {
                match mainstate {
                    CollectState::BuildKey => {
                        key.push(e_char);
                    }
                    CollectState::BuildVal => {
                        value.push(e_char);
                    }
                    _ => {
                        return Err(ParseErr::OUT_STATE);
                    }
                }
                substate = StrParseState::Default;
            }
            StrParseState::InStr => match mainstate {
                CollectState::BuildKey => match e_char {
                    '\"' => {
                        key.push(e_char);
                        substate = StrParseState::OutStr;
                    }
                    '\\' => {
                        substate = StrParseState::InStrSpecial;
                    }
                    _ => {
                        key.push(e_char);
                    }
                },
                CollectState::BuildVal => match e_char {
                    '\"' => {
                        value.push(e_char);
                        mainstate = CollectState::PrepKey;
                        substate = StrParseState::Default;
                    }
                    '\\' => {
                        substate = StrParseState::InStrSpecial;
                    }
                    _ => {
                        value.push(e_char);
                    }
                },
                _ => {
                    return Err(ParseErr::OUT_STATE);
                }
            },
            StrParseState::OutStr => match mainstate {
                CollectState::BuildKey => match e_char {
                    ' ' => {
                        continue;
                    }
                    ':' => {
                        mainstate = CollectState::PrepVal;
                        substate = StrParseState::Default;
                    }
                    _ => {
                        return Err(ParseErr::INVL_KEY_FMT);
                    }
                },
                _ => {
                    return Err(ParseErr::OUT_STATE);
                }
            },
            StrParseState::InStrSpecial => {
                match mainstate {
                    CollectState::BuildKey => {
                        key.push(e_char);
                    }
                    CollectState::BuildVal => {
                        value.push(e_char);
                    }
                    _ => {
                        return Err(ParseErr::OUT_STATE);
                    }
                }
                substate = StrParseState::InStr;
            }
        }
    }
    // Any remaining string key and string value shall be added to hashmap
    if key.len() > 0 && value.len() > 0 {
        hm.insert(key.clone(), value.clone());
    } else {
        return Err(ParseErr::INVL_VAL_FMT);
    }
    return Ok(hm);
}