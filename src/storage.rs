use std::{fs::{File, OpenOptions}, io::ErrorKind};

pub fn load_kv_file () -> File {
    let kv_file_result = OpenOptions::new()
    .read(true)
    .append(true)
    .create(true)
    .open(".log.jsonl");

    let kv_file = match kv_file_result {
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

    kv_file
}