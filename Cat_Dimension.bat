@echo off
cd C:\\RustProjects\\Cat_Dimension
cargo build --release
copy /Y target\\release\\Cat_Dimension.exe "%~dp0"
cd /d "%~dp0"
Cat_Dimension.exe
pause
del Cat_Dimension.exe