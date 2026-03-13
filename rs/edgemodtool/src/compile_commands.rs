use edgemodtoolcore::loc::Loc;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

pub fn compile_command(path: PathBuf) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("The path {} does not exist.", path.display()));
    }

    if !path.is_file() {
        return Err(format!("The path {} is not a file.", path.display()));
    }

    match path.file_name().unwrap().to_str().unwrap() {
        "text.json" => compile_text_loc(path)?,
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

pub fn decompile_command(path: PathBuf) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("The path {} does not exist.", path.display()));
    }

    if !path.is_file() {
        return Err(format!("The path {} is not a file.", path.display()));
    }

    match path.file_name().unwrap().to_str().unwrap() {
        "text.loc" => decompile_text_loc(path)?,
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

fn compile_text_loc(path: PathBuf) -> Result<(), String> {
    println!("Compiling text.loc");

    let text_json = File::open(&path).map_err(|err| format!("{}", err))?;
    let br = BufReader::new(text_json);
    let loc = Loc::from_json(br).map_err(|err| format!("{}", err))?;
    let output_path = path.with_extension("loc");
    let text_text_loc = File::create(output_path).map_err(|err| format!("{}", err))?;
    let bw = BufWriter::new(text_text_loc);

    loc.save_text_loc(bw).map_err(|err| format!("{}", err))
}

fn decompile_text_loc(path: PathBuf) -> Result<(), String> {
    println!("Decompiling text.loc");

    let text_loc = File::open(&path).map_err(|err| format!("{}", err))?;
    let br = BufReader::new(text_loc);
    let loc = Loc::from_text_loc(br).map_err(|err| format!("{}", err))?;
    let output_path = path.with_extension("json");
    let text_json = File::create(output_path).map_err(|err| format!("{}", err))?;
    let bw = BufWriter::new(text_json);

    loc.save_json(bw).map_err(|err| format!("{}", err))
}
