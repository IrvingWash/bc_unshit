use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

use id3::{Tag, TagLike};

use crate::utils;

const MP3_EXTENSION: &str = "mp3";

pub struct Unshitter {
    album_path: PathBuf,
    destination_path: PathBuf,
    should_clean: bool,
}

impl Unshitter {
    pub fn new(album_path: PathBuf, destination_path: PathBuf, should_clean: bool) -> Self {
        Self {
            album_path,
            destination_path,
            should_clean,
        }
    }

    pub fn go(&self) -> Result<(), String> {
        Unshitter::move_and_unshit(&self.album_path, &self.destination_path)?;

        if self.should_clean {
            utils::remove_dir(&self.album_path)?
        }

        Ok(())
    }

    fn find_album_metainfo(path: &PathBuf) -> Result<Option<(String, String, String)>, String> {
        let dir = fs::read_dir(path).map_err(utils::error_to_string)?;

        let mut sub_dirs: Vec<DirEntry> = Vec::new();

        for entry in dir {
            let entry = entry.map_err(utils::error_to_string)?;

            if entry.file_type().map_err(utils::error_to_string)?.is_dir() {
                sub_dirs.push(entry);

                continue;
            }

            if utils::is_same_extension(entry.file_name(), MP3_EXTENSION) {
                let tags = Tag::read_from_path(entry.path()).map_err(utils::error_to_string)?;

                match (tags.artist(), tags.album(), tags.year()) {
                    (Some(artist), Some(album), Some(year)) => {
                        return Ok(Some((
                            artist.to_owned(),
                            album.to_owned(),
                            year.to_string(),
                        )))
                    }
                    _ => return Ok(None),
                };
            };
        }

        for sub_dir in sub_dirs {
            if let Some(metainfo) = Unshitter::find_album_metainfo(&sub_dir.path())? {
                return Ok(Some(metainfo));
            }
        }

        Ok(None)
    }

    fn move_and_unshit(source: &PathBuf, destination: &Path) -> Result<(), String> {
        let dir = fs::read_dir(source).map_err(utils::error_to_string)?;

        let (artist_name, album_name, year) = Unshitter::find_album_metainfo(source)?.unwrap_or((
            String::from(""),
            String::from(""),
            String::from(""),
        ));

        let destination = destination
            .join(artist_name)
            .join(format!("{} - {}", year, album_name));

        utils::create_dir_if_not_exists(&destination)?;

        for entry in dir {
            let entry = entry.map_err(utils::error_to_string)?;

            // Recursively handle a directory
            if entry.file_type().map_err(utils::error_to_string)?.is_dir() {
                Unshitter::move_and_unshit(&entry.path(), &destination.join(entry.path()))?;

                continue;
            }

            // Handle a non-mp3 file
            if !utils::is_same_extension(entry.file_name(), MP3_EXTENSION) {
                fs::copy(entry.path(), destination.join(entry.file_name()))
                    .map_err(utils::error_to_string)?;

                continue;
            }

            // Handle an mp3 file
            let tags = Tag::read_from_path(entry.path()).map_err(utils::error_to_string)?;

            if let (Some(track_number), Some(title)) = (tags.track(), tags.title()) {
                fs::copy(
                    entry.path(),
                    destination.join(format!(
                        "{}. {}.mp3",
                        track_number,
                        title.replace('\0', "-")
                    )),
                )
                .map_err(utils::error_to_string)?;
            };
        }

        Ok(())
    }
}
