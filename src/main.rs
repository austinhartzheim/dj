use std::process::Command;
use std::process::Stdio;
use std::env;
use std::io;

fn main() {

    let mut args =  env::args();

    let mainarg = args.nth(1);
    if mainarg.is_none() {
        panic!("Need to specify more parameters");
    }

    match mainarg.unwrap().as_ref() {
        "test" => run_tests(),
        "coverage" => run_coverage(),
        _ => panic!("Invalid parameter"),
    }

}


fn run_tests() {
    println!("Running tests...");

    let output = Command::new("python3")
        .arg("manage.py")
        .arg("test")
        .stdout(Stdio::inherit())
        .output()
        .unwrap_or_else(|e| {
            panic!("Failed to execute process: {}", e)
        });
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));

}


fn run_coverage() {
    // Run tests with coverage tracking enabled
    println!("Running tests with coverage...");

    let output = Command::new("coverage")
        .arg("run")
        .arg("manage.py")
        .arg("test")
        .stdout(Stdio::inherit())
        .output()
        .unwrap_or_else(|e| {
            panic!("Failed to execute process: {}", e)
        });
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));

    // Display the coverage report
    let report = Command::new("coverage")
        .arg("report")
        .stdout(Stdio::inherit())
        .output()
        .unwrap_or_else(|e| {
            panic!("Failed to execute process: {}", e)
        });
    println!("{}", String::from_utf8_lossy(&output.stdout));
}