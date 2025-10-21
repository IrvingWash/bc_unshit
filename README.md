# bandcamp unshittifier

![Crates.io Version](https://img.shields.io/crates/v/bc_unshit)


This app takes a path to an album downloaded from [bandcamp](https://bandcamp.com) and changes it's structure and renames the files.

# Contents
- [What it does](#how-it-works)<br>
- [Installation](#installation)<br>
- [Command-line Options](#command-line-options)<br>
- [Additional links](#additional-links)<br>

## How it works
Typical bandcamp album structure:
```
~/Downloads
├── Krallice - Ygg Huur
  ├── Krallice - 01 Idols.mp3
  ├── Krallice - 02 Wastes of Ocean.mp3
  ├── Krallice - 03 Over Spirit.mp3
  ├── Krallice - 04 Tyranny of Thought.mp3
  ├── Krallice - 05 Bitter Meditation.mp3
  ├── Krallice - 06 Engram.mp3
  ├── cover.png
```
Sucks, doesn't it?
Now run:
```bash
bc_unshit --source ~/Downloads/Krallice\ -\ Ygg\ Huur --destination ~/Music --remove-source
```
And behold the beauty:
```
~/Music
├── Krallice
  ├── 2016 - Ygg Huur
    ├── 01. Idols.mp3
    ├── 02. Wastes of Ocean.mp3
    ├── 03. Over Spirit.mp3
    ├── 04. Tyranny of Thought.mp3
    ├── 05. Bitter Meditation.mp3
    ├── 06. Engram.mp3
    ├── cover.png
```
Run `bc_unshit --help` to learn more.

## Installation
```
cargo install bc_unshit
```

## Command-line options
```bash
-s, --source <SOURCE>            # Path to the downloaded album (should be unzipped)
-d, --destination <DESTINATION>  # Path to the destination directory
-r, --remove-source              # Should the source folder be deleted
-h, --help                       # Print help
-V, --version                    # Print version
```

## Additional links
[Check out my bandcamp downloader](https://github.com/IrvingWash/arsene)
