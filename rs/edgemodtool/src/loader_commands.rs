use crate::Result;
use edgemodtoolcore::loader::*;

pub fn install_loader_command(profile: Option<String>) -> Result<()> {
    if let Some(profile) = &profile {
        println!("Installing loader to profile '{}'", profile);
    } else {
        println!("Installing loader to default profile");
    }

    install_loader(profile.as_ref()).map_err(handle_errors)?;

    println!("Install successful");

    Ok(())
}

pub fn uninstall_loader_command(profile: Option<String>) -> Result<()> {
    if let Some(profile) = &profile {
        println!("Uninstalling loader to profile '{}'", profile);
    } else {
        println!("Uninstalling loader to default profile");
    }

    uninstall_loader(profile.as_ref()).map_err(handle_errors)?;

    println!("Uninstall successful");

    Ok(())
}

fn handle_errors(e: (LoaderErrors, String)) -> String {
    e.1
}
