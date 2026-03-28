use crate::profiles::{ProfilesErrors, read_profile_json};
use std::env::current_exe;
use std::fs::{File, create_dir, remove_dir_all, remove_file};
use std::path::PathBuf;
use zip::ZipArchive;
use zip::result::ZipError;

define_errors_enum! {
    LoaderErrors;
    PathDoesNotExist = 1,
    NotAValidEdgeDirectory = 2,
    ArchiveDoesNotExist = 3,
    LoaderAlreadyInstalled = 4,
    LoaderNotInstalled = 5,
    IOError = 6,
    ProfileDoesNotExist = 7,
}

pub fn install_loader(profile: &str) -> Result<(), (LoaderErrors, String)> {
    let profiles = read_profile_json().map_err(handle_io_errors)?;

    let edgeexe = profiles.get_path_for_profile(profile).map_err(|e| {
        if e == ProfilesErrors::ProfileDoesNotExist {
            (LoaderErrors::ProfileDoesNotExist, format!("{:?}", LoaderErrors::ProfileDoesNotExist))
        } else {
            panic!("Unknown error")
        }
    })?;

    let edgepath = edgeexe.parent().unwrap();

    if !edgepath.exists() {
        return Err((LoaderErrors::PathDoesNotExist, format!("The path {} does not exist.", edgepath.display())));
    }

    if !edgepath.is_dir() || !edgeexe.exists() {
        return Err((
            LoaderErrors::NotAValidEdgeDirectory,
            format!("The path {} is not a valid EDGE directory.", edgepath.display()),
        ));
    }

    let mut mods_path: PathBuf = edgepath.into();

    mods_path.push("mods");

    if mods_path.exists() && mods_path.is_dir() {
        return Err((LoaderErrors::LoaderAlreadyInstalled, "The loader is already installed.".to_string()));
    }

    let loader_archive = current_exe().unwrap().with_file_name("loader.zip");

    if !loader_archive.exists() || !loader_archive.is_file() {
        return Err((LoaderErrors::ArchiveDoesNotExist, "The loader archive does not exist.".to_string()));
    }

    let loader_archive = File::open(loader_archive).map_err(handle_io_errors)?;
    let mut loader_archive = ZipArchive::new(loader_archive).map_err(handle_zip_errors)?;

    loader_archive.extract(edgepath).map_err(handle_zip_errors)?;
    create_dir(edgeexe.with_file_name("mods")).map_err(handle_io_errors)?;

    Ok(())
}

pub fn uninstall_loader(profile: &str) -> Result<(), (LoaderErrors, String)> {
    let profiles = read_profile_json().map_err(handle_io_errors)?;

    let edgeexe = profiles.get_path_for_profile(profile).map_err(|e| {
        if e == ProfilesErrors::ProfileDoesNotExist {
            (LoaderErrors::ProfileDoesNotExist, format!("{:?}", LoaderErrors::ProfileDoesNotExist))
        } else {
            panic!("Unknown error")
        }
    })?;

    let edgepath = edgeexe.parent().unwrap();

    if !edgepath.exists() {
        return Err((LoaderErrors::PathDoesNotExist, format!("The path {} does not exist.", edgepath.display())));
    }

    if !edgepath.is_dir() || !edgeexe.exists() {
        return Err((
            LoaderErrors::NotAValidEdgeDirectory,
            format!("The path {} is not a valid EDGE directory.", edgepath.display()),
        ));
    }

    let mods_path = edgeexe.with_file_name("mods");

    if !mods_path.exists() || !mods_path.is_dir() {
        return Err((LoaderErrors::LoaderNotInstalled, "The loader is not installed.".to_string()));
    }

    remove_dir_all(mods_path).map_err(handle_io_errors)?;
    remove_dir_all(edgeexe.with_file_name("plugins")).map_err(handle_io_errors)?;
    remove_file(edgeexe.with_file_name("xinput1_3.dll")).map_err(handle_io_errors)?;

    Ok(())
}

fn handle_io_errors(err: std::io::Error) -> (LoaderErrors, String) {
    (LoaderErrors::IOError, err.to_string())
}

fn handle_zip_errors(e: ZipError) -> (LoaderErrors, String) {
    (LoaderErrors::IOError, e.to_string())
}
