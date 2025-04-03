use std::fs::File;
use std::io::{BufRead, BufReader};

use log_viewer::log_record::LogRecord;

fn main() {
    let log_file = BufReader::new(File::open("xilem_log.json").unwrap());
    for line in log_file.lines() {
        let line = line.unwrap();
        let record: LogRecord = serde_json::from_str(&line).unwrap();
        println!("{record:?}");
    }
}
