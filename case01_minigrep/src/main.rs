use std::{env, process};
use case01_minigrep::Config;

fn main() {
    // 接收非法字符 std::env::args_os // OsString
    // println!("{:?}", args);

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    /*println!("Search for {}", query);
    println!("In file {}", filename);*/

    if let Err(e) = case01_minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}