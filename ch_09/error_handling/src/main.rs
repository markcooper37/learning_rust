use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() { // Try to create the file if it doesn't exist
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

// This function uses error propagation
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("goodbye.txt")?.read_to_string(&mut s)?;
    Ok(s)
    // This could be replaced with 'use std::fs;' and the following single line:
    // fs::read_to_string("hello.txt")
}