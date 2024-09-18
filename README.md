# bandcamp unshittifier

This app takes a path to an album downloaded from [bandcamp](https://bandcamp.com) and changes it structure and renames the files.

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
