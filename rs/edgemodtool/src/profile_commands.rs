use edgemodtoolcore::profiles::Profiles;
use std::env::current_exe;
use std::fs::{File, write};
use std::io::{BufReader, BufWriter, stdin, stdout, Write};
use std::path::PathBuf;
use std::process::Command;

pub fn add_profile_command(name: Option<String>, path: Option<PathBuf>) -> Result<(), String> {
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

    if !path.exists() {
        return Err(format!("The path {} does not exist.", path.display()));
    }

    if !path.is_file() {
        return Err(format!("The path {} is not a file.", path.display()));
    }

    if !path.ends_with("edge.exe") {
        return Err(format!("The path {} is not a valid EDGE executable.", path.display()));
    }

    let profile_json = get_profile_json_path()?;

    if !profile_json.exists() {
        write(
            &profile_json,
            r#"{
    "default": "",
    "profiles": {}
}"#,
        )
        .map_err(|err| format!("{}", err))?;
    }

    let mut profiles = read_profile_json()?;
    let current_count = profiles.get_count();

    profiles.add_or_modify_profile(&name, path);

    if current_count == 0 {
        profiles.set_default(&name)?;
    }

    write_profile_json(profiles)?;

    println!("Successfully added/modified profile {}.", name);

    Ok(())
}

pub fn remove_profile_command(name: String) -> Result<(), String> {
    let mut profiles = read_profile_json()?;

    profiles.remove_profile(&name)?;

    if profiles.get_count() == 0 {
        profiles.set_default("")?;
    }

    write_profile_json(profiles)?;

    println!("Successfully removed profile {}.", name);

    Ok(())
}

pub fn default_profile_command(name: String) -> Result<(), String> {
    let mut profiles = read_profile_json()?;

    profiles.set_default(&name)?;
    write_profile_json(profiles)?;

    println!("Setting default profile: {}", name);

    Ok(())
}

pub fn launch_command(profile: Option<String>) -> Result<(), String> {
    let profiles = read_profile_json()?;

    let profile = if let Some(profile) = profile {
        profile
    } else {
        profiles.get_default().to_string()
    };

    let edgeexe = profiles.get_path_for_profile(&profile)?;

    println!("Launching profile '{}' ({})", profile, edgeexe.display());

    Command::new(edgeexe).spawn().map_err(|err| format!("{}", err))?;

    Ok(())
}

fn get_profile_json_path() -> Result<PathBuf, String> {
    let profile_json = current_exe().map_err(|err| format!("{}", err))?;
    let mut profile_json = profile_json.parent().ok_or("Unknown Error")?.to_path_buf();

    profile_json.push("profile.json");

    Ok(profile_json)
}

fn read_profile_json() -> Result<Profiles, String> {
    let file = File::open(&get_profile_json_path()?).map_err(|err| format!("{}", err))?;
    let br = BufReader::new(file);
    let profiles = Profiles::from_json(br).map_err(|err| format!("{}", err))?;

    Ok(profiles)
}

fn write_profile_json(profiles: Profiles) -> Result<(), String> {
    let file = File::create(&get_profile_json_path()?).map_err(|err| format!("{}", err))?;
    let bw = BufWriter::new(file);

    profiles.save_json(bw).map_err(|err| format!("{}", err))?;

    Ok(())
}
