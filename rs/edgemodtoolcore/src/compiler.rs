use crate::loc::Loc;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

define_errors_enum! {
    CompilerErrors;
    OpenError = 1,
    WriteError = 2,
    SerializationError = 3,
    CompileError = 4,
}

pub fn compile_text_loc(path: PathBuf) -> Result<(), (CompilerErrors, String)> {
    let text_json = File::open(&path).map_err(|err| (CompilerErrors::OpenError, err.to_string()))?;
    let br = BufReader::new(text_json);
    let loc = Loc::from_json(br).map_err(|err| (CompilerErrors::SerializationError, err.to_string()))?;
    let output_path = path.with_extension("loc");
    let text_text_loc = File::create(output_path).map_err(|err| (CompilerErrors::WriteError, err.to_string()))?;
    let bw = BufWriter::new(text_text_loc);

    loc.save_text_loc(bw).map_err(|err| (CompilerErrors::CompileError, err.to_string()))
}

pub fn decompile_text_loc(path: PathBuf) -> Result<(), (CompilerErrors, String)> {
    let text_loc = File::open(&path).map_err(|err| (CompilerErrors::OpenError, err.to_string()))?;
    let br = BufReader::new(text_loc);
    let loc = Loc::from_text_loc(br).map_err(|err| (CompilerErrors::SerializationError, err.to_string()))?;
    let output_path = path.with_extension("json");
    let text_json = File::create(output_path).map_err(|err| (CompilerErrors::WriteError, err.to_string()))?;
    let bw = BufWriter::new(text_json);

    loc.save_json(bw).map_err(|err| (CompilerErrors::CompileError, err.to_string()))
}
