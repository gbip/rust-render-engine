use std;
use std::fs::File;
use std::io::{Write, Read, Error};

pub fn write_string_to_file(j: &str, file_name: String) -> std::io::Result<()> {
    let mut file = File::create(file_name).unwrap();
    file.write_all(j.as_bytes())
}

#[allow(unused_must_use)]
pub fn open_file_as_string(file: &str) -> Result<String, Error> {
    let mut result: String = "".to_string();
    match File::open(file) {
        Ok(mut val) => {
            val.read_to_string(&mut result);
            Ok(result)
        },
        Err(e) => Err(e),
    }
}
