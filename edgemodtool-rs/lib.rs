use crate::loc::Loc;
use std::ffi::{CStr, c_char};
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub mod loc;

#[unsafe(no_mangle)]
extern "C" fn decompile_text_loc(text_loc_path: *const c_char, json_path: *const c_char) -> i32 {
    if text_loc_path.is_null() || json_path.is_null() {
        return -1;
    }

    let Ok(text_loc_path) = unsafe { CStr::from_ptr(text_loc_path) }.to_str() else {
        return 1;
    };

    let Ok(f1) = File::open(text_loc_path) else {
        return 2;
    };

    let br = BufReader::new(f1);

    let Ok(loc) = Loc::from_text_loc(br) else {
        return 3;
    };

    let Ok(json_path) = unsafe { CStr::from_ptr(json_path) }.to_str() else {
        return 1;
    };

    let Ok(f2) = File::create(json_path) else {
        return 2;
    };

    let bw = BufWriter::new(f2);

    if loc.save_json(bw).is_err() {
        return 3;
    }

    0
}

#[unsafe(no_mangle)]
extern "C" fn compile_text_loc(json_path: *const c_char, text_loc_path: *const c_char) -> i32 {
    if json_path.is_null() || text_loc_path.is_null() {
        return -1;
    }

    let Ok(json_path) = unsafe { CStr::from_ptr(json_path) }.to_str() else {
        return 1;
    };

    let Ok(f1) = File::open(json_path) else {
        return 2;
    };

    let br = BufReader::new(f1);

    let Ok(loc) = Loc::from_json(br) else {
        return 3;
    };

    let Ok(text_loc_path) = unsafe { CStr::from_ptr(text_loc_path) }.to_str() else {
        return 1;
    };

    let Ok(f2) = File::create(text_loc_path) else {
        return 2;
    };

    let bw = BufWriter::new(f2);

    if loc.save_text_loc(bw).is_err() {
        return 3;
    };

    0
}
