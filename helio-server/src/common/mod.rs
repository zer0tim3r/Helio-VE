use std::process::Command;

pub fn create_process(path: &str, args: Vec<&str>) -> std::process::Output {
    let output: std::process::Output = Command::new(path)
        .args(args)
        .output()
        .expect("Failed to execute command");

    output
}