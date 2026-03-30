@echo off
setlocal enabledelayedexpansion

echo ===== Rake Build ^& Install =====

REM Check for required tools
echo Checking dependencies...

where cargo >nul 2>nul
if errorlevel 1 (
    echo Error: Rust/Cargo is not installed
    echo Visit https://www.rust-lang.org/tools/install
    exit /b 1
)

where cmake >nul 2>nul
if errorlevel 1 (
    echo Error: CMake is not installed
    echo Visit https://cmake.org/download/
    exit /b 1
)

where cl.exe >nul 2>nul
if errorlevel 1 (
    echo Error: Microsoft C++ compiler not found
    echo Please install Visual Studio Build Tools or Visual Studio Community
    exit /b 1
)

REM Get the directory where this script is located
set SCRIPT_DIR=%~dp0
cd /d "%SCRIPT_DIR%"

REM Create build directory
echo Creating build directory...
if not exist "build" mkdir build
cd build

REM Run CMake
echo Running CMake...
cmake .. -G "Visual Studio 16 2019" -DCMAKE_BUILD_TYPE=Release
if errorlevel 1 (
    echo CMake failed
    exit /b 1
)

REM Build
echo Building project...
cmake --build . --config Release -j %NUMBER_OF_PROCESSORS%
if errorlevel 1 (
    echo Build failed
    exit /b 1
)

REM Determine install prefix
set INSTALL_PREFIX=%PROGRAMFILES%\rake
if not defined INSTALL_PREFIX set INSTALL_PREFIX=%USERPROFILE%\AppData\Local\rake

REM Install
echo Installing to %INSTALL_PREFIX%...
if not exist "%INSTALL_PREFIX%\bin" mkdir "%INSTALL_PREFIX%\bin"
if not exist "%INSTALL_PREFIX%\lib" mkdir "%INSTALL_PREFIX%\lib"

copy Release\rake.exe "%INSTALL_PREFIX%\bin\" >nul
copy ..\rake_parser\target\release\rake_parser.dll "%INSTALL_PREFIX%\lib\" >nul 2>nul

REM Add to PATH
echo Adding to PATH...
setx PATH "%PATH%;%INSTALL_PREFIX%\bin"

REM Copy Rakefile if it doesn't exist
if not exist "%INSTALL_PREFIX%\Rakefile" (
    copy ..\Rakefile "%INSTALL_PREFIX%\" >nul 2>nul
)

echo.
echo ===== Installation complete! =====
echo.
echo To start using rake:
echo   1. Open a new Command Prompt or PowerShell
echo   2. Run: rake --^<section^>
echo.
pause
