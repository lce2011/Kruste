@echo off

echo Applying permission...
wsl -d archlinux bash -ic "chmod +x ./helper/quick-sharun"

echo Preparing patchelf using pacman...
wsl -d archlinux bash -ic "pacman -Syu patchelf xvfb-run"

echo Setting enviroment variables...
wsl -d archlinux bash -ic "export APPDIR=./AppDir"
wsl -d archlinux bash -ic "export ICON=../assets/kruste-icon.png"
wsl -d archlinux bash -ic "export DESKTOP=../assets/kruste.desktop"
wsl -d archlinux bash -ic "export OUTPATH=../.dist/"
wsl -d archlinux bash -ic "export OUTNAME=kruste-$(uname -m).AppImage"

echo Applying quick-sharun...
wsl -d archlinux bash -ic "./helper/quick-sharun ../target/release/kruste"
echo Packaging quick-sharun AppImage...
wsl -d archlinux bash -ic "./helper/quick-sharun --make-appimage"

echo Moving AppImage...
wsl -d archlinux bash -ic "mv ./*.AppImage ../target/release/anylinux-x86_64.AppImage"

echo Done. Check the target/release/ folder for the AppImage.