use std::{collections::{HashMap, HashSet}, fs::{File, OpenOptions}, io::{ BufRead, BufReader, ErrorKind, Write}, time::{Duration, SystemTime, UNIX_EPOCH}};
use serde::{Deserialize, Serialize};
use serde_json::{Error};

use crate::command::Command;
use crate::error::KVError;
use crate::time::has_passed;

#[derive(Serialize, Deserialize)]
pub struct KV {
    op: String,
    key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<u64>
}

#[derive(Debug, Clone)]
pub struct KVValue {
    value: Option<Vec<u8>>,
    expires_at: Option<u64>
}
#[derive(Debug)]
pub enum KVResult {
    Get { value: Option<KVValue> },
    Set,
    Delete,
    Has { has: bool },
    Persist,
    Touch,
}


impl KV {
    pub fn build(command: &Command, file:  &mut File, kvstore: &mut HashMap<String, KVValue>) -> Result<KVResult, KVError> {
        match command {
            Command::Set { key, value, ttl } => {
                let expires_at: Option<u64> = ttl.map(|time| SystemTime::now()
                    .checked_add(Duration::from_secs(time))
                    .unwrap()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                );
    
                let kv = KV {op: "set".to_string(), key: key.clone(), value: Some(value.clone()), expires_at: expires_at };
                match serde_json::to_string(&kv) { 
                    Ok(json_string) => {

                        kvstore.insert(key.clone(), KVValue { value: kv.value, expires_at: kv.expires_at });
                        if let Err(_) = writeln!(file, "{}", json_string) {
                            return Err(KVError::TestErr)
                        }
                        file.flush().expect("Failed to flush file");
                        return Ok(KVResult::Set);
                    },
                    Err(_) => return Err(KVError::TestErr)
                };

            },
            Command::Get { key } => {
                let expired = match kvstore.get(key) {
                    Some(kv_value) => kv_value.expires_at.map_or(false, |expires_at| has_passed(expires_at)),
                    None => {
                        eprintln!("Key not found");
                        return Ok(KVResult::Get { value: None });
                    }
                };
                if expired {
                    let _ = KV::build(&Command::Delete { key: key.clone() }, file, kvstore);
                }
    
                if let Some(kv_value) = kvstore.get(key) {
                    // if let Ok(value_str) = String::from_utf8(kv_value.value.clone().unwrap()) {
                        return Ok(KVResult::Get { value: Some(kv_value.clone()) as Option<KVValue> });
                        //println!("{} expires at {:?}", value_str, kv_value.expires_at);
                    // } else {
                    //     println!("{:?}", kv_value.value);
                    // }
                }
                Ok(KVResult::Get { value: None })
            },
            Command::Delete { key } => {
                // check if it exists
                if kvstore.contains_key(key){
                    let kv = KV {op: "del".to_string(), key: key.clone(), value: None, expires_at: None };
                    match serde_json::to_string(&kv) { 
                        Ok(json_string) => {
                            kvstore.remove(key);
                            if let Err(_) = writeln!(file, "{}", json_string) {
                                eprintln!("Couldn't write to file");
                                return Err(KVError::TestErr);
                            }
                            let _ = file.flush();
                        },
                        Err(_) => eprintln!("error")
                    };
                }
                Ok(KVResult::Delete)
            },
            Command::Has { key } => {
                if let Some(kv_value) = kvstore.get(key) {
                    println!("{} -> {:?}", key, String::from_utf8(kv_value.value.clone().unwrap()));
                }
                else {
                    println!("false");
                }

                Ok(KVResult::Has { has: true })
            },
            Command::Persist { key } => {
                if let Ok(kv_result) = KV::build(&Command::Get { key: key.clone() }, file, kvstore) {
                    match kv_result {
                        KVResult::Get { value } => {
                            if let Some(val) = value{
                            let kv = KV {op: "set".to_string(), key: key.clone(), value: val.value.clone(), expires_at: None };
                                match serde_json::to_string(&kv) { 
                                    Ok(json_string) => {
                                        kvstore.insert(key.clone(), KVValue { value: val.value, expires_at: None });
                                        if let Err(_) = writeln!(file, "{}", json_string) {
                                            return Err(KVError::TestErr)
                                        }
                                        file.flush().expect("Failed to flush file");
                                    },
                                    Err(_) => {
                                        println!("Error");
                                        return Err(KVError::TestErr)
                                    }
                                };    
                            }    
                        },
                        _ => {println!("code"); return Ok(KVResult::Persist)}
                    }
                }                    
                
                Ok(KVResult::Persist)
            },
            Command::Touch { key, ttl } => {
                if let Ok(kv_result) = KV::build(&Command::Get { key: key.clone() }, file, kvstore) {
                    match kv_result {
                        KVResult::Get { value } => {
                            if let Some(val) = value{
                            let expires_at: u64 = SystemTime::now()
                                .checked_add(Duration::from_secs(*ttl))
                                .unwrap()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs();
                            let kv = KV {op: "set".to_string(), key: key.clone(), value: val.value.clone(), expires_at: Some(expires_at) };
                            match serde_json::to_string(&kv) { 
                                Ok(json_string) => {
                                    kvstore.insert(key.clone(), KVValue { value: val.value, expires_at: Some(expires_at) });
                                    if let Err(_) = writeln!(file, "{}", json_string) {
                                        return Err(KVError::TestErr)
                                    }
                                    file.flush().expect("Failed to flush file");
                                },
                                Err(_) => {
                                    println!("Error");
                                    return Err(KVError::TestErr)
                                }
                            };        
                        }
                        },
                        _ => {println!("code"); return Ok(KVResult::Touch)}
                    }
                }                    
                
                Ok(KVResult::Touch)
            },
        }
    }


    pub fn load_to_mem(file: &File) -> Result<HashMap<String, KVValue>, String>{
        let mut data_mem: HashMap<String, KVValue> = HashMap::new();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(json_str) => {
                    let kv: Result<KV, Error> = serde_json::from_str(json_str.as_str());
                    match kv {
                        Ok(kv) => {
                            if kv.op == "set" {
                                match kv.expires_at {
                                    Some(expires_at)=> if !has_passed(expires_at) {
                                        let value_map: KVValue = KVValue { value: kv.value, expires_at: kv.expires_at };
                                        data_mem.insert(kv.key, value_map);
                                    },
                                    None => {
                                        let value_map: KVValue = KVValue { value: kv.value, expires_at: kv.expires_at };
                                        data_mem.insert(kv.key, value_map);
                                    }
                                };
                            }
                            else if kv.op == "del" {
                                data_mem.remove(&kv.key);
                            }
                        },
                        Err(_) => eprintln!("Invalid json line found")
                    }
                },
                Err(_) => return Err(format!("Failed to interpret line"))
            }
            
        }
        Ok(data_mem)
    }

    pub fn compact(kvstore: &HashMap<String, KVValue>) -> std::io::Result<()> {
        let new_file_result = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(".log-0001.jsonl");
        
        let mut new_kv_file = match new_file_result {
            Ok(file) => file,
            Err(err) => {
                match err.kind() {
                    ErrorKind::NotFound => {
                        match File::create(".log.jsonl") {
                            Ok(fc) => fc,
                            Err(_) => panic!("Problem occured while creating kv file.")
                        }
                    },
                    _ => panic!("Problem occured while opening/creating kv file.")
                }
            }
        };

        let mut keys_added = HashSet::new();
        for (key, kv_value) in kvstore {
            if let Some(expires_at) = kv_value.expires_at {
                if has_passed(expires_at) {
                    continue;
                }
            }
            let kv = KV {op: "set".to_string(), key: key.clone(), value: kv_value.value.clone(), expires_at: kv_value.expires_at };
            if let Ok(json_string) = serde_json::to_string(&kv) { 
                writeln!(new_kv_file, "{}", json_string)?;
                keys_added.insert(kv.key.clone());
            }
        }
        new_kv_file.sync_all()?;
        std::fs::rename(".log-0001.jsonl", ".log.jsonl")?;
        let dir = File::open(".")?;
        dir.sync_all()?;

        Ok(())
    }
}
