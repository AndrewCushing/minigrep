use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{Read};
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage:");
        println!("minigrep search_string filename");
    }

    run(args);
}
