#[macro_use]
mod macros;
pub mod compiler;
pub mod loc;
pub mod mod_definition;
pub mod profiles;

use std::ffi::{CStr, c_char};
use std::path::PathBuf;
use std::str::FromStr;

#[unsafe(no_mangle)]
extern "C" fn compile_text_loc(json_path: *const c_char) -> i32 {
    if json_path.is_null() {
        return -1;
    }

    let Ok(json_path) = unsafe { CStr::from_ptr(json_path) }.to_str() else {
        return -2;
    };

    match compiler::compile_text_loc(json_path.into()) {
        Ok(_) => 0,
        Err(e) => e.0.into(),
    }
}

#[unsafe(no_mangle)]
extern "C" fn decompile_text_loc(text_loc_path: *const c_char) -> i32 {
    if text_loc_path.is_null() {
        return -1;
    }

    let Ok(text_loc_path) = unsafe { CStr::from_ptr(text_loc_path) }.to_str() else {
        return -2;
    };

    match compiler::decompile_text_loc(text_loc_path.into()) {
        Ok(_) => 0,
        Err(e) => e.0.into(),
    }
}

#[unsafe(no_mangle)]
extern "C" fn add_profile(name: *const c_char, path: *const c_char) -> i32 {
    if name.is_null() || path.is_null() {
        return -1;
    }

    let Ok(name) = unsafe { CStr::from_ptr(name) }.to_str() else {
        return -2;
    };

    let Ok(path) = unsafe { CStr::from_ptr(path) }.to_str() else {
        return -2;
    };

    let path = PathBuf::from_str(path).unwrap();

    match profiles::add_profile(name, &path) {
        Ok(_) => 0,
        Err(e) => e.0.into(),
    }
}

#[unsafe(no_mangle)]
extern "C" fn remove_profile(name: *const c_char) -> i32 {
    if name.is_null() {
        return -1;
    }

    let Ok(name) = unsafe { CStr::from_ptr(name) }.to_str() else {
        return -2;
    };

    match profiles::remove_profile(name) {
        Ok(_) => 0,
        Err(e) => e.0.into(),
    }
}

#[unsafe(no_mangle)]
extern "C" fn default_profile(name: *const c_char) -> i32 {
    if name.is_null() {
        return -1;
    }

    let Ok(name) = unsafe { CStr::from_ptr(name) }.to_str() else {
        return -2;
    };

    match profiles::default_profile(name) {
        Ok(_) => 0,
        Err(e) => e.0.into(),
    }
}

#[unsafe(no_mangle)]
extern "C" fn launch(profile: *const c_char) -> i32 {
    if profile.is_null() {
        return -1;
    }

    let Ok(profile) = unsafe { CStr::from_ptr(profile) }.to_str() else {
        return -2;
    };

    match profiles::launch(Some(&profile.to_string())) {
        Ok(_) => 0,
        Err(e) => e.0.into(),
    }
}
