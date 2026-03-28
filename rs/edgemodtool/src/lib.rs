mod commands;
mod compile_commands;
mod loader_commands;
mod profile_commands;

pub use commands::*;
pub use compile_commands::*;
pub use loader_commands::*;
pub use profile_commands::*;
use std::ffi::{CStr, CString, c_char};
use std::ptr::null_mut;

pub type Result<T> = std::result::Result<T, String>;

#[link(name = "edgemodtoolcore.dll")]
unsafe extern "C" {
    fn get_default_profile(name: *mut *mut c_char) -> i32;
    fn delete_string(ptr: *mut c_char) -> i32;
}

fn get_default_profile_name(profile: Option<String>) -> Result<String> {
    if let Some(profile) = profile {
        Ok(profile)
    } else {
        unsafe {
            let mut ptr: *mut c_char = null_mut();

            if get_default_profile(&mut ptr) != 0 {
                return Err("Unknown Error.".to_string());
            }

            let value = CString::from(CStr::from_ptr(ptr)).to_string_lossy().to_string();

            delete_string(ptr);

            Ok(value)
        }
    }
}
