# Kruste - ***K**eyboard-based **Rust**y **E**ditor*

*Kruste is an command-line, keyboard only text editor written in Rust using Ratatui & Crossterm.*

<hr>

### Installation & Building

**Windows** Go to the Release tab and download the latest `kruste.exe`

**UNIX** Go to the Release tab and download the latest Linux binary for your libc. Support exists for:
- x86_64 glibc
- x86_64 musl-libc
- aarch64 Android BIONIC

### Buiding

**Windows**

1. Clone the project: `git clone https://github.com/lce2011/Kruste.git`
2. Go to the directory and build the project: `.\scripts\full-build.bat`
3. The Kruste binary now sits inside `.\target\release`

**UNIX**

1. Clone the project: `git clone https://github.com/lce2011/Kruste.git`
2. Go to the directory and build the project: `cargo build --release`
3. The Kruste binary now sits inside `./target/release/` or `./target/debug/`, incase you built without the `--release` flag.

<hr>

### Open a file

`kruste <path>`

> [!NOTE]
> If the file at the given path doesn't exist, Kruste automatically creates the file.

<hr>

### Customization

Kruste will look for an custom configuration at the path set in the `KRUSTE_CONFIG` enviroment variable.

I recommend using the following location for your Kruste configuration:

**Windows** `C:\Users\<user>\.config\kruste\config.json`

**UNIX** `~/.config/kruste/config.json`

For the full customization docs, see [here](./docs/CUSTOMIZATION.md)

> [!WARNING]
> Kruste doesn't automatically generate any config.json file, even if it says, it uses the default config. The actual config.json has to be created seperatly.

<hr>

### Bugs & Issues

If you have a bug, please open up an Issue and describe, what happened and how to replicate the bug.
