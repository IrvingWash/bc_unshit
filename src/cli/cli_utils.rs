use std::path::PathBuf;

use expanduser::expanduser;

use crate::utils::error_to_string;

pub fn parse_tilde(path: &str) -> Result<PathBuf, String> {
    expanduser(path).map_err(error_to_string)
}
