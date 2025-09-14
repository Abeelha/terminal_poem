@echo off
echo Building Terminal Poems viewer...
cargo build --release
echo.
echo Build complete! 
echo Executable location: target\release\poems.exe
echo.
echo To run: target\release\poems.exe
pause