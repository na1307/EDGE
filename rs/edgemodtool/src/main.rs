use clap::Parser;
use edgemodtool::*;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Compile { path } => compile_command(path),
        Commands::Decompile { path } => decompile_command(path),
        Commands::InstallLoader { profile } => Ok(()),
        Commands::UninstallLoader { profile } => Ok(()),
        Commands::Install { path, profile } => Ok(()),
        Commands::Uninstall { modname, profile } => Ok(()),
        Commands::Profile { command } => match command {
            ProfileCommands::Add { name, path } => add_profile_command(name, path),
            ProfileCommands::Remove { name } => remove_profile_command(name),
            ProfileCommands::Default { name } => default_profile_command(name),
        },
        Commands::Launch { profile } => launch_command(profile),
    };

    if let Err(error) = result {
        eprintln!("ERROR: {}", error);
    }
}
