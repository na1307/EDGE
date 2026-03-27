mod commands;
mod compile_commands;
mod profile_commands;

pub use commands::*;
pub use compile_commands::*;
pub use profile_commands::*;

pub type Result<T> = std::result::Result<T, String>;
