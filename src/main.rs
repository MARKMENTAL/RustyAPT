use std::io::prelude::*;
use std::io;
use std::process::Command;
use std::fs::OpenOptions;

fn install_package() {
    let mut package_name = String::new();
    println!("Enter package name:");
    io::stdin().read_line(&mut package_name).unwrap();
    let install_command = format!("sudo apt-get -y install {}", package_name.trim());

    let output = Command::new("sh")
        .arg("-c")
        .arg(install_command)
        .output()
        .expect("Failed to execute command");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout_string = String::from_utf8_lossy(&output.stdout);
    if !stdout_string.contains("0 upgraded, 0 newly installed") {
        save_change(format!("Installed package: {}", package_name.trim()));
    }
}


fn remove_package() {
    let mut package_name = String::new();
    println!("Enter package name:");
    io::stdin().read_line(&mut package_name).unwrap();
    let remove_command = format!("sudo apt-get -y remove {}", package_name.trim());

    let output = Command::new("sh")
        .arg("-c")
        .arg(remove_command)
        .output()
        .expect("Failed to execute command");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout_string = String::from_utf8_lossy(&output.stdout);
    if !stdout_string.contains("0 to remove") {
        save_change(format!("Removed package: {}", package_name.trim()));
    }
}



fn update_repos() {
    let upgrade_command = "sudo apt-get update";

    let output = Command::new("sh")
        .arg("-c")
        .arg(upgrade_command)
        .output()
        .expect("Failed to execute command");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

fn upgrade_system() {
    let upgrade_command = "sudo apt-get upgrade";

    let output = Command::new("sh")
        .arg("-c")
        .arg(upgrade_command)
        .output()
        .expect("Failed to execute command");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

fn save_change(change: String) {
    let file_name = "package_changes.txt";
    let mut file = match OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_name)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Couldn't open or create file: {}", e);
            return;
        }
    };
    if let Err(e) = writeln!(file, "{}", change) {
        eprintln!("Couldn't write to file: {}", e);
    }
}


fn main() {
    let mut option = String::new();
    println!("Welcome to Rusty APT: The Debian apt-get interface written in Rust");
    println!("1. Install package\n2. Remove package\n3. Update repos\n4. Upgrade system\nEnter option:");
    io::stdin().read_line(&mut option).unwrap();
    match option.trim() {
        "1" => install_package(),
        "2" => remove_package(),
        "3" => update_repos(),
        "4" => upgrade_system(),
        _ => println!("Invalid option"),
    }
}

