use std::{fs, path::PathBuf};

pub fn copy_dir(source: PathBuf, destination: PathBuf) -> Result<(), String> {
    if !fs::exists(&destination).map_err(|e| e.to_string())? {
        fs::create_dir_all(&destination).map_err(|e| e.to_string())?;
    };

    let dir = fs::read_dir(source).map_err(|e| e.to_string())?;

    for entry in dir {
        let entry = entry.map_err(|e| e.to_string())?;

        if !entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            fs::copy(entry.path(), destination.join(entry.file_name()))
                .map_err(|e| e.to_string())?;
        } else {
            copy_dir(entry.path(), destination.join(entry.file_name()))?;
        }
    }

    Ok(())
}
