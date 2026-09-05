#!/bin/bash

echo "Setting up function 'is_termux()'..."
is_termux() {
    [[ -n "${TERMUX_VERSION:-}" ]] || [[ "${PREFIX:-}" == *"com.termux"* ]]
}

echo "[====== LOCAL BUILDS (cargo-local) ======]"
echo "Building for local platform (UNIX)..."

if [ -f /etc/arch-release ]; then
    echo "Archlinux detected. Building just once..."
    cargo build --release
    if [$? != 0]; then
        echo "Build failed."
        exit 1
    fi

else
    echo "No Archlinux detected. First build..."
    cargo build --release
    if [ $? -ne 0 ]; then
        echo "Build failed."
        exit 1
    fi

    echo "Second build using PRoot Distro archlinux/archlinux container..."
    proot-distro login archlinux --bind "$PWD:$PWD" -- bash -c "cd '$PWD' && cargo build --release"
    if [ $? -ne 0 ]; then
        echo "Build failed."
        exit 1
    fi
fi

if [ is_termux ]; then
    echo "Detected Termux. Skipping cargo-cross builds..."
else
    echo "[====== CROSS BUILDS (cargo-cross) ======]"
    echo "Extra build for x86_64-pc-windows-gnu (Windows)..."
    cargo-cross build --release --target x86_64-pc-windows-gnu
    mv ../target/x86_64-pc-windows-gnu/release/kruste.exe ../target/x86_64-pc-windows-gnu/release/x86_64-windows-kruste.exe
    if [ $? -ne 0 ]; then
        echo "Build failed."
        exit 1
    fi

    echo "Building for x86_64-unknown-linux-gnu (glibc)..."
    cargo-cross build --release --target x86_64-unknown-linux-gnu
    mv ../target/x86_64-unknown-linux-gnu/release/kruste ../target/x86_64-unknown-linux-gnu/release/x86_64-glibc-kruste
    if [ $? -ne 0 ]; then
        echo "Build failed."
        exit 1
    fi

    echo "Building for x86_64-unknown-linux-musl (musl-libc)..."
    cargo-cross build --release --target x86_64-unknown-linux-musl
    mv ../target/x86_64-unknown-linux-musl/release/kruste ../target/x86_64-unknown-linux-musl/release/x86_64-musl-kruste
    if [ $? -ne 0 ]; then
        echo "Build failed."
        exit 1
    fi

    echo "Building for aarch64-linux-android (Android BIONIC)..."
    cargo-cross build --release --target aarch64-linux-android
    mv ../target/aarch64-linux-android/release/kruste ../target/aarch64-linux-android/release/aarch64-android-bionic-kruste
    if [ $? -ne 0 ]; then
        echo "Build failed."
        exit 1
    fi
fi

echo "Done. Check inside target/."