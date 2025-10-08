@echo off
REM Clean build artifacts

echo Cleaning build artifacts...

REM Remove the book folder
if exist "book" (
    echo Removing book folder...
    rmdir /s /q "book"
)

REM Remove Rust build artifacts
if exist "target" (
    echo Removing Rust build artifacts...
    rmdir /s /q "target"
)

if exist "src\target" (
    echo Removing Rust build artifacts...
    rmdir /s /q "src\target"
)

echo Clean completed!
pause
