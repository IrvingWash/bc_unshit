use std::{
    error::Error,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

pub fn create_dir_if_not_exists(path: &PathBuf) -> Result<(), String> {
    if !fs::exists(path).map_err(error_to_string)? {
        fs::create_dir_all(path).map_err(error_to_string)?;
    }

    Ok(())
}

pub fn is_same_extension(filename: OsString, expected_ext: &str) -> bool {
    if let Some(ext) = Path::new(&filename).extension() {
        return match ext.to_str() {
            None => false,
            Some(ext) => ext == expected_ext,
        };
    }

    false
}

pub fn remove_dir(path: &PathBuf) -> Result<(), String> {
    fs::remove_dir_all(path).map_err(error_to_string)?;

    Ok(())
}

pub fn error_to_string(e: impl Error) -> String {
    e.to_string()
}
