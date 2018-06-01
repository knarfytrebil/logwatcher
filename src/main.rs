use std::env::args;
use std::sync::mpsc;
use std::process::exit;

extern crate logwatcher;
use logwatcher::LogWatcher;

fn main(){

    let (tx, rx) = mpsc::channel();

    let filename = match args().nth(1) {
        Some(x) => x,
        None => {
            println!("File name required");
            exit(1);
        }
    };

    let mut log_watcher = LogWatcher::register(filename).unwrap();
    log_watcher.register_channel(tx);

    log_watcher.watch(&|line: String| {
        println!("Line {}", line);
    });

}
