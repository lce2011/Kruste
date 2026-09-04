@echo off

echo [====== LOCAL BUILDS (cargo-local)======]
echo Building for local platform (Windows)...
cargo build --release
if %errorlevel% neq 0 (
    echo Build failed.
    exit /b 1
)

echo Building for local archlinux (WSL2 archlinux - used for the AppImage)
wsl -d archlinux bash -ic "cargo build --release"
if %errorlevel% neq 0 (
    echo Build failed.
    exit /b 1
)

echo [====== CROSS BUILDS (cargo-cross) ======]
echo Building for x86_64-unknown-linux-gnu (glibc)...
cross build --release --target x86_64-unknown-linux-gnu
mv ../target/x86_64-unknown-linux-gnu/release/kruste ../target/x86_64-unknown-linux-gnu/release/x86_64-glibc-kruste
if %errorlevel% neq 0 (
    echo Build failed.
    exit /b 1
)

echo Building for x86_64-unknown-linux-musl (musl-libc)...
cross build --release --target x86_64-unknown-linux-musl
mv ../target/x86_64-unknown-linux-musl/release/kruste ../target/x86_64-unknown-linux-musl/release/x86_64-musl-kruste
if %errorlevel% neq 0 (
    echo Build failed.
    exit /b 1
)

echo Building for aarch64-linux-android (Android BIONIC)...
cross build --release --target aarch64-linux-android
mv ../target/aarch64-linux-android/release/kruste ../target/aarch64-linux-android/release/aarch64-android-bionic-kruste
if %errorlevel% neq 0 (
    echo Build failed.
    exit /b 1
)

echo Done. Check inside target/.