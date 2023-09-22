use std::process::Command;
use crate::{models::Project};
use anyhow::{Result};


pub fn rust_starter(project: Project) -> Result<()> {
    // Check if cargo is installed
    let cargo_check = Command::new("cargo")
        .arg("--version")
        .output();

    match cargo_check {
        Ok(output) => {
            if output.status.success() {
                println!("Cargo is installed: {:?}", String::from_utf8_lossy(&output.stdout));
            } else {
                eprintln!("Cargo is not installed.");
                // Install cargo
                install_cargo();
            }
        }
        Err(_) => {
            eprintln!("Cargo is not installed.");
            // Install cargo
            install_cargo();
        }
    }

    println!("Creating project directory.");

    let create_project_cmd = format!("mkdir apps/{}", project.name);
    run_bash_command(&create_project_cmd);

    let change_dir_cmd = format!("cd {}", project.name);
    run_bash_command(&change_dir_cmd);

    println!("Initializing Rust Project...");
    let init_project_cmd = "cargo init";
    run_bash_command(init_project_cmd);
    
    Ok(())
}


fn run_bash_command(cmd: &str) {

    match Command::new("bash").arg("-c").arg(cmd).status() {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("Command '{}' executed successfully!", cmd);
            } else {
                eprintln!("Command '{}' failed with exit code: {}", cmd, exit_status.code().unwrap_or(-1));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command '{}': {}", cmd, e);
        }
    }
}


fn install_cargo() {
    println!("Installing Cargo...");
    
    let status = 
            Command::new("sh")
                .arg("-c")
                .arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")
                .status();

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("Cargo installed successfully.");
            } else {
                eprintln!("Failed to install Cargo.");
            }
        }
        Err(_) => {
            eprintln!("Failed to execute the installation command.");
        }
    }
}