use console::style;
use shlex::split;
use std::{error::Error, process::Command};

pub fn run_command(command: &str, command_name: &str) -> Result<String, Box<dyn Error>> {
    let command_args: Vec<String> = split(command).unwrap();
    let output = Command::new(&command_args[0])
        .args(&command_args[1..])
        .output();
    if output.is_ok() {
        let output = output?;
        if output.status.success() {
            if command_name != "" {
                println!("{} {}", style("[OK]").green(), command_name);
            }
            return Ok(String::from_utf8(output.stdout)?);
        } else {
            let err = String::from_utf8(output.stderr)?;
            if command_name != "" {
                println!(
                    "{} {}: {}",
                    style("[ERROR]").red(),
                    command_name,
                    err.to_string()
                );
            }
            return Err(err.into());
        }
    } else {
        if command_name != "" {
            println!("{} {}", style("[ERROR]").red(), command_name);
        }
        return Err(output.err().unwrap().into());
    }
}
