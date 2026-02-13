# Media Rename Tool

A Rust-based CLI tool to mass rename media files for Movies and TV Shows.

## Features

- **Movie Mode (`--movie`)**: Formats files to `Title (Year).ext`.
- **TV Mode (`--tv`)**: Formats files to `Title SxxExx.ext`.
- **String Replacement**: Support for finding and replacing specific text using `--replace <FIND> <REPLACE>`.
- **Cross-Platform**: Works on Windows, macOS, and Linux.

## Installation

### Prerequisites
Ensure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

### Building from Source

1. Clone the repository:
   ```bash
   git clone <repository_url>
   cd media-rename-tool
   ```

2. Install globally using Cargo:
   ```bash
   cargo install --path .
   ```

## Usage

Run the tool from the directory containing your media files.

### Windows

**Open PowerShell or Command Prompt in the target directory:**

```powershell
media-rename-tool.exe --movie
# OR
media-rename-tool.exe --tv
```

### macOS & Linux

**Open Terminal in the target directory:**

```bash
media-rename-tool --movie
# OR
media-rename-tool --tv
```

### Examples

#### Movie Mode
Renames files by keeping the title and year, wrapping the year in parentheses.

- Input: `Barbie 2019 XDLolRips 1080p BRAP.mkv`
- Command: `media-rename-tool --movie`
- Output: `Barbie (2019).mkv`

#### TV Mode
Renames files by keeping the title and `SxxExx` identifier, removing subsequent text.

- Input: `Schitt's Creek S03E12.720p.WEB-DL.x265-HETeam.mkv`
- Command: `media-rename-tool --tv`
- Output: `Schitt's Creek S03E12.mkv`

#### Find & Replace
Replaces specific text in the filename before formatting.

- Input: `SchittsCreek.S04E10.mkv`
- Command: `media-rename-tool --tv --replace "SchittsCreek" "Schitt's Creek"`
- Output: `Schitt's Creek S04E10.mkv`
