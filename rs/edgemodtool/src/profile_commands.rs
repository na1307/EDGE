use crate::Result;
use edgemodtoolcore::profiles::*;
use std::io::{Write, stdin, stdout};
use std::path::PathBuf;

pub fn add_profile_command(name: Option<String>, path: Option<PathBuf>) -> Result<()> {
    let name = if let Some(name) = name {
        name
    } else {
        let mut buffer = String::new();

        print!("Profile name: ");

        stdout().flush().map_err(|err| format!("{}", err))?;
        stdin().read_line(&mut buffer).map_err(|err| format!("{}", err))?;

        buffer.trim().to_string()
    };

    if name.is_empty() {
        return Err("The name cannot be empty.".to_string());
    }

    let path = if let Some(path) = path {
        path
    } else {
        let mut buffer = String::new();

        print!("edge.exe path: ");

        stdout().flush().map_err(|err| format!("{}", err))?;
        stdin().read_line(&mut buffer).map_err(|err| format!("{}", err))?;

        buffer.trim().to_string().into()
    };

    add_profile(&name, &path).map_err(handle_errors)?;

    println!("Successfully added/modified profile {}.", name);

    Ok(())
}

pub fn remove_profile_command(name: String) -> Result<()> {
    remove_profile(&name).map_err(handle_errors)?;

    println!("Successfully removed profile {}.", name);

    Ok(())
}

pub fn default_profile_command(name: String) -> Result<()> {
    default_profile(&name).map_err(handle_errors)?;

    println!("Setting default profile: {}", name);

    Ok(())
}

pub fn launch_command(profile: Option<String>) -> Result<()> {
    if let Some(profile) = &profile {
        println!("Launching profile '{}'", profile);
    } else {
        println!("Launching default profile");
    }

    launch(profile.as_ref()).map_err(handle_errors)?;

    Ok(())
}

fn handle_errors(e: (ProfilesErrors, String)) -> String {
    e.1
}
