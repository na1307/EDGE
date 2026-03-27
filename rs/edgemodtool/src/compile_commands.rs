use crate::Result;
use std::path::PathBuf;
use edgemodtoolcore::compiler::{compile_text_loc, decompile_text_loc};

pub fn compile_command(path: PathBuf) -> Result<()> {
    if !path.exists() {
        return Err(format!("The path {} does not exist.", path.display()));
    }

    if !path.is_file() {
        return Err(format!("The path {} is not a file.", path.display()));
    }

    match path.file_name().unwrap().to_str().unwrap() {
        "text.json" => {
            println!("Compiling text.loc");

            compile_text_loc(path).map_err(|e| e.1)?
        }
        _ => {
            return Err(format!(
                "The file \"{}\" is the unknown file name for currently not supported.",
                path.display()
            ));
        }
    };

    println!("Successfully compiled the file.");

    Ok(())
}

pub fn decompile_command(path: PathBuf) -> Result<()> {
    if !path.exists() {
        return Err(format!("The path {} does not exist.", path.display()));
    }

    if !path.is_file() {
        return Err(format!("The path {} is not a file.", path.display()));
    }

    match path.file_name().unwrap().to_str().unwrap() {
        "text.loc" => {
            println!("Decompiling text.loc");

            decompile_text_loc(path).map_err(|e| e.1)?
        }
        _ => {
            return Err(format!(
                "The file \"{}\" is the unknown file name for currently not supported.",
                path.display()
            ));
        }
    };

    println!("Successfully decompiled the file.");

    Ok(())
}
