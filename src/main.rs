use std::env;
use std::process::Command;

fn main() {
    let output = Command::new("ls")
        .arg("-halF")
        .arg(env::var("HOME").unwrap_or(".".to_string()))
        .output()
        .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
