# Building

As this project uses Rust via `cargo` and also has custom Build-Scripts, building should be quite simple.

> [!WARNING]
> The reason for no Build-Scripts existing for Linux/UNIX-systems is, that the Build-Scripts install/update the neccessary packages like `cargo` themselves, but every system uses a different package
> manager, so it isn't possible to make an universal Build-Script that also works, if you don't have `cargo` or similar packages yet.

> [!WARNING]
> Regarding the above reason for no Linux/UNIX Build-Scripts, the Windows Build-Scripts require WSL2 with Archlinux being installed.

## Windows

You have two options for building on Windows:

### Option 1: Commandline:
1. Clone the project using `git clone https://github.com/lce2011/Kruste.git`
2. Go to the directory and build the project using `cargo build --release`
3. Then check in `target/release/` for the `kruste.exe` file (or `target/debug/` if you build without the `--release` flag).

### Option 2: Custom Build-Script:
- First run the `full-build.bat` script inside `scripts/`. This builds for Windows, WSL2 Archlinux, Linux glibc, Linux musl-libc and Android BIONIC.
- Then run `package-appimage.bat` script inside `scripts/` to generate the universal AppImage for all Linux glibc & musl-libc distros using the WSL2 Archlinux build.
- All the executables sit inside `target/release/` and other subfolders of `target/`.

> [!INFO]
> If you just want the Windows build, just use the first Option.
> If you want to contribute or want to build cross-platform, then use the second Option.

## UNIX

For UNIX systems like MacOS or Linux, there are no custom Build-Scripts. Instead you should use the default `cargo build` command for building. For that, follow the steps below:

1. Clone the project using `git clone https://github.com/lce2011/Kruste.git`
2. Go to the directory and build the project using `cargo build --release`
3. The Kruste binary now sits inside `./target/release/` or `./target/debug/`, incase you built without the `--release` flag.