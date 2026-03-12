use clap::{Parser, Subcommand};
use edgemodtoolcore::loc::Loc;
use edgemodtoolcore::profiles::Profiles;
use std::env::current_exe;
use std::fs::{File, write};
use std::io::{BufReader, BufWriter, Write, stdin, stdout};
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(name = "EdgeModTool", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Compile a file
    Compile {
        /// File path
        path: PathBuf,
    },
    /// Decompile a file
    Decompile {
        /// File path
        path: PathBuf,
    },
    /// Install a mod
    Install {
        /// Mod path
        path: PathBuf,
        /// Profile name
        #[arg(long)]
        profile: Option<String>,
    },
    /// Uninstall a mod
    Uninstall {
        /// Mod name
        modname: String,
        /// Profile name
        #[arg(long)]
        profile: Option<String>,
    },
    /// Profile actions
    Profile {
        #[command(subcommand)]
        command: ProfileCommands,
    },
    /// Launch the game
    Launch {
        /// Profile name
        profile: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ProfileCommands {
    /// Add or Modify a profile
    Add {
        /// Profile name
        name: Option<String>,
        /// edge.exe path
        path: Option<PathBuf>,
    },
    /// Remove a profile
    Remove {
        /// Profile name
        name: String,
    },
    /// Change the default profile
    Default {
        /// Profile name
        name: String,
    },
}

pub fn compile_command(path: PathBuf) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("The path {} does not exist.", path.display()));
    }

    if !path.is_file() {
        return Err(format!("The path {} is not a file.", path.display()));
    }

    match path.file_name().unwrap().to_str().unwrap() {
        "text.json" => {
            println!("Compiling text.loc");

            let text_json = File::open(&path).map_err(|err| format!("{}", err))?;
            let br = BufReader::new(text_json);
            let loc = Loc::from_json(br).map_err(|err| format!("{}", err))?;
            let output_path = path.with_extension("loc");
            let text_text_loc = File::create(output_path).map_err(|err| format!("{}", err))?;
            let bw = BufWriter::new(text_text_loc);

            loc.save_text_loc(bw).map_err(|err| format!("{}", err))?;
        }
        _ => {
            return Err(format!(
                "The file {} is the unknown file name for currently not supported.",
                path.display()
            ));
        }
    }

    println!("Successfully compiled the file.");

    Ok(())
}

pub fn decompile_command(path: PathBuf) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("The path {} does not exist.", path.display()));
    }

    if !path.is_file() {
        return Err(format!("The path {} is not a file.", path.display()));
    }

    match path.file_name().unwrap().to_str().unwrap() {
        "text.loc" => {
            println!("Decompiling text.loc");

            let text_loc = File::open(&path).map_err(|err| format!("{}", err))?;
            let br = BufReader::new(text_loc);
            let loc = Loc::from_text_loc(br).map_err(|err| format!("{}", err))?;
            let output_path = path.with_extension("json");
            let text_json = File::create(output_path).map_err(|err| format!("{}", err))?;
            let bw = BufWriter::new(text_json);

            loc.save_json(bw).map_err(|err| format!("{}", err))?;
        }
        _ => {
            return Err(format!(
                "The file {} is the unknown file name for currently not supported.",
                path.display()
            ));
        }
    }

    println!("Successfully decompiled the file.");

    Ok(())
}

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
