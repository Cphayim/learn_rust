mod read_username;

use std::{fs::File, io::ErrorKind};

use read_username::read_username_from_file;

fn main() {
    // let filename = "argo.toml";
    // let file_result = File::open(&filename);
    // let file = match file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create(&filename) {
    //             Ok(file) => file,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    match read_username_from_file() {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    };
}
