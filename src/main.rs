// struct KVStore {
//     data: HashMap<String, String>
// }

// impl KVStore {
//     fn set(&mut self, key: String, value: String) {
//         self.data.insert(key, value);
//     }
//     fn get(&self, key: &str) {
//         self.data.get(key);
//     }
//     fn delete(&mut self, key: &str) {
//         self.data.remove(key);
//     }    
// }

mod command;
mod config;
mod kv;
mod storage;
mod time;
mod error;

use regex::Regex;

use crate::config::Config;
use crate::kv::KV;
use crate::storage::load_kv_file;

use std::{ env, io::{self, Write}};





fn main() {

    let mut log = load_kv_file();
    let mut kvstore= KV::load_to_mem(&log).unwrap();

    let args: Vec<String> = env::args().collect();
    let command = Config::build(&args);
    
    if let Ok(config) = command {
        let _ = KV::build(&config.command, &mut log, &mut kvstore);
        return;
    }


          
    loop {
        print!("kvstore> ");
        io::stdout().flush().unwrap();
        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");
        let cmd = cmd.trim();

        let re = Regex::new(r"^(\S+)\s+(\S+)(?:\s+(.+?))?(?:\s+(\d+))?$").unwrap();
        if let Some(caps) = re.captures(cmd) {
            let args: Vec<String> = caps.iter()
            .skip(1)
            .filter(|m| m.is_some())
            .map(|m| m.unwrap().as_str().to_string())
            .collect(); // cmd.split(' ').map(|val| val.to_string()).collect();
            let command = Config::build_with_loop(&args);

            let cmd = match command {
                Ok(cmd) => cmd.command, 
                Err(err) => { 
                    eprintln!("{}", err); 
                    continue;
                }
            };
            let _ = KV::build(&cmd, &mut log, &mut kvstore);
        }

        // if cmd == "exit" {
        //     break;
        // }

        // let args: Vec<String> = cmd.split(' ').map(|val| val.to_string()).collect();
        // let command = Config::build_with_loop(&args);

        // let cmd = match command {
        //     Ok(cmd) => cmd.command, 
        //     Err(err) => { 
        //         eprintln!("{}", err); 
        //         continue;
        //     }
        // };
        // let _ = KV::build(&cmd, &mut log, &mut kvstore);
        if let Ok(metadata) = log.metadata() {
            //println!("{}", metadata.len());
            if metadata.len() > 1_048_576 {
                let _ = KV::compact(&kvstore);
            }
        }
    }
}
