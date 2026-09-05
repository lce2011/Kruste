# Building

As this project uses Rust via `cargo` and also has custom Build-Scripts, building should be quite simple.

> [!WARNING]
> There is no package-appimage.sh script for UNIX-like systems, because the tested version was broken.
> **Solution is WIP!**

## Dependencies

Ofcourse you need `cargo`, but you will also require `cargo-cross`, which can be installed via `cargo install cargo-cross`.

You will also need `WSL2's archlinux` (Windows) or `proot-distro's archlinux/archlinux container`, which also need to have `cargo` installed. These can be installed like this:

- `wsl --install -d archlinux`
- `proot-distro install archlinux/archlinux`

<hr>

## Windows

You have two options for building on Windows:

### Option 1: Manual command:
1. Clone the project using `git clone https://github.com/lce2011/Kruste.git`
2. Go to the directory and build the project using `cargo build --release`
3. Then check in `target/release/` for the `kruste.exe` file (or `target/debug/` if you build without the `--release` flag).

### Option 2: Custom Build-Script:
1. Run the `full-build.bat` script inside `scripts/`. This builds for Windows, WSL2 Archlinux, Linux glibc, Linux musl-libc and Android BIONIC.
2. Run `package-appimage.bat` script inside `scripts/` to generate the universal AppImage for all Linux glibc & musl-libc distros using the WSL2 Archlinux build.
3. All the executables sit inside `target/release/` and other sub-folders of `target/`.

> [!NOTE]
> If you just want the Windows build, just use the first Option.
> If you want to contribute or want to build cross-platform, then use the second Option.

## UNIX

For UNIX systems like MacOS or Linux, you can use the command like below or use the full-build.sh script. Instead you should use the default `cargo build` command for building. For that, follow the steps below:

1. Clone the project using `git clone https://github.com/lce2011/Kruste.git`
2. Go to the directory and build the project using `cargo build --release`
3. The Kruste binary now sits inside `./target/release/` or `./target/debug/`, in case you built without the `--release` flag.

> [!NOTE]
> You can't package the AppImage on Linux/UNIX. That can only be done on Windows.