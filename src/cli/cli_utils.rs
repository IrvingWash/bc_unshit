use std::path::PathBuf;

use expanduser::expanduser;

pub fn parse_tilde(path: &str) -> Result<PathBuf, String> {
    expanduser(path).map_err(|e| e.to_string())
}
