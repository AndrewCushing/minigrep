use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{Read};
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("2 command line args required")
    }

    run(args);
}
