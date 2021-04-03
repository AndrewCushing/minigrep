use std::fs::File;
use std::path::Path;
use std::io::{Read};

pub fn run(args: Vec<String>) {
    let searching_for = &args[1];
    let file_path = &args[2];

    println!("File path given: {}", file_path);
    println!("Searching for string: {}", searching_for);
    println!();

    let mut file = match File::open(Path::new(file_path)) {
        Ok(i) => i,
        Err(_) => panic!("Could not open file {}", file_path)
    };

    let mut contents: String = String::new();

    let read_result = file.read_to_string(&mut contents);

    match read_result {
        Ok(_) => (),
        Err(d) => panic!("Couldn't read stuff from file. Details: {:?}", d)
    }

    let lines = contents.split_terminator("\n");

    let mut line_num = 1;
    for x in lines {
        if x.contains(searching_for) {
            println!("Matched line {}:", line_num);
            println!("{}", x);
        }
        line_num += 1;
    }
}