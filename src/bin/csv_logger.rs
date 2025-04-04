use std::io::Write;
use std::fs::OpenOptions;
use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let file_path = "log.csv"; // let CLI options handle this later

    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Unable to open file");

    println!("Logging to '{}'", file_path);

    for line in handle.lines() {
        match line {
            Ok(value) => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs_f64();

                let entry = format!("{:.3},{}\n", now, value.trim());
                if let Err(e) = file.write_all(entry.as_bytes()) {
                    eprintln!("Write failed: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
            }
        }
    }
}