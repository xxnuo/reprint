@echo off
cargo build --release
cp .\target\release\r.exe r.exe
upx r.exe