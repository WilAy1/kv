use std::{process::exit};

use crate::command::Command;

pub struct Config {
    pub command: Command
}

impl Config {
    pub fn build (args: &[String]) -> Result<Config, String> {
        if args.len() < 4 {
            return Err(format!("Invalid command structure"));
        }
        if args[1].as_str() != "kvstore" {
            return Err(format!("Invalid command structure"));
        } else if args[2].as_str() == "set" && args.len() < 5 {
            return Err(format!("Invalid command. Use kvstore set <key> <value>"))
        } else if (args[2].as_str() == "get" || args[2].as_str() == "delete") && args.len() != 4 {
            return Err(format!(
                "Invalid command. Use kvstore {} <key>",
                args[2]
            ));
        }



        let command = match args[2].as_str() {
            "set" => Command::Set { key: args[3].clone(), value: Vec::from(args[4].clone()), ttl: if args.len() >=6 {args[5].trim().parse::<u64>().ok() } else { None } },
            "get" => Command::Get { key: args[3].clone() },
            "delete" => Command::Delete { key: args[3].clone() }, // Delete
            "has" | "exists" => Command::Has { key: args[3].clone() }, // Check if exists
            "touch" => Command::Touch { key: args[3].clone(), ttl: args[4].trim().parse::<u64>().unwrap() }, // Change Expiration
            "persist" => Command::Persist { key: args[3].clone() }, // Remove Expiration
            "ttl" => Command::Ttl { key: args[3].clone() },

            _ => return Err(format!("Invalid command provided"))
        };

        Ok(Config { command })
    }

    pub fn build_with_loop (args: &Vec<String>) -> Result<Config, String> {
        // if args.len() < 4 {
        //     return Err("Invalid command structure");
        // }
        // println!("{}", args[1]);
        // if args[0].as_str() != "kvstore" {
        //     return Err(format!("Invalid command structure"));
        // } else 

        // assert_eq!((args[0].as_str() == "get" || args[0].as_str() == "delete") && args.len() != 2, true);
        if args[0].as_str() == "set" && args.len() < 3 {
            return Err(format!("Invalid command. Use set <key> <value> <ttl, optional>"))
        } else if (args[0].as_str() == "get" || args[0].as_str() == "has" || args[0].as_str() == "delete") && args.len() != 2 {
            return Err(format!(
                "Invalid command. Use {} <key>",
                args[0]
            ));
        }


        let command = match args[0].as_str() {
            "set" => Command::Set { key: args[1].clone(), value: Vec::from(args[2].clone()), ttl: if args.len() >=4 {args[3].trim().parse::<u64>().ok() } else { None }  },
            "get" => Command::Get { key: args[1].clone() },
            "delete" => Command::Delete { key: args[1].clone() },
            "has" | "exists" => Command::Has { key: args[1].clone() },
            "touch" => Command::Touch { key: args[1].clone(), ttl: args[2].trim().parse::<u64>().unwrap() },
            "persist" => Command::Persist { key: args[1].clone() },
            "ttl" => Command::Ttl { key: args[1].clone() },
            "exit" => exit(0),
            _ => return Err(format!("Invalid command provided"))
        };

        Ok(Config { command })
    }
}