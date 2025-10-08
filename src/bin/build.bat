@echo off
REM NervaWeb Interactive Build Script for Windows

REM Navigate to project root
cd /d "%~dp0\..\.."

REM Check for command line arguments
set QUIET_MODE=0
set SELECTED_LANG=
set SELECTED_THEME=
set SELECTED_COMMAND=

:parse_args
if "%1"=="" goto end_parse
if "%1"=="--quiet" (
    set QUIET_MODE=1
    shift
    goto parse_args
)
if "%1"=="-q" (
    set QUIET_MODE=1
    shift
    goto parse_args
)
if "%1"=="--lang" (
    set SELECTED_LANG=%2
    shift
    shift
    goto parse_args
)
if "%1"=="-l" (
    set SELECTED_LANG=%2
    shift
    shift
    goto parse_args
)
if "%1"=="--theme" (
    set SELECTED_THEME=%2
    shift
    shift
    goto parse_args
)
if "%1"=="-t" (
    set SELECTED_THEME=%2
    shift
    shift
    goto parse_args
)
if "%1"=="--help" (
    echo NervaWeb Interactive Build Script
    echo.
    echo Usage: %0 [OPTIONS]
    echo.
    echo Options:
    echo   -q, --quiet     Quiet mode
    echo   -l, --lang LANG Language selection
    echo   -t, --theme THEME Theme selection
    echo   -h, --help      Show this help
    goto end
)
if "%1"=="-h" (
    echo NervaWeb Interactive Build Script
    echo.
    echo Usage: %0 [OPTIONS]
    echo.
    echo Options:
    echo   -q, --quiet     Quiet mode
    echo   -l, --lang LANG Language selection
    echo   -t, --theme THEME Theme selection
    echo   -h, --help      Show this help
    goto end
)
shift
goto parse_args
:end_parse

REM Interactive menu (only if no command line arguments)
if %QUIET_MODE%==0 if "%SELECTED_LANG%"=="" if "%SELECTED_COMMAND%"=="" (
    echo.
    echo ====================================
    echo     NervaWeb Interactive Builder
    echo ====================================
    echo.

    REM Language selection
    :lang_menu
    echo 🌐 Choose Language / Выберите язык:
    echo 1. 🇷🇺 Русский (ru) - Russian
    echo 2. 🇺🇸 English (en) - English
    echo 3. 🇪🇸 Español (es) - Spanish
    echo 4. 🇩🇪 Deutsch (de) - German
    echo 5. 🇫🇷 Français (fr) - French
    echo 6. 🇮🇹 Italiano (it) - Italian
    echo 7. 🇵🇹 Português (pt) - Portuguese
    echo 8. 🇨🇳 中文 (zh) - Chinese
    echo 9. 🇯🇵 日本語 (ja) - Japanese
    echo 10. 🇰🇷 한국어 (ko) - Korean
    echo 11. 🇸🇦 العربية (ar) - Arabic
    echo 12. 🇮🇳 हिन्दी (hi) - Hindi
    echo.
    set /p lang_choice="Enter language number (1-12): "

    if "%lang_choice%"=="1" set SELECTED_LANG=ru
    if "%lang_choice%"=="2" set SELECTED_LANG=en
    if "%lang_choice%"=="3" set SELECTED_LANG=es
    if "%lang_choice%"=="4" set SELECTED_LANG=de
    if "%lang_choice%"=="5" set SELECTED_LANG=fr
    if "%lang_choice%"=="6" set SELECTED_LANG=it
    if "%lang_choice%"=="7" set SELECTED_LANG=pt
    if "%lang_choice%"=="8" set SELECTED_LANG=zh
    if "%lang_choice%"=="9" set SELECTED_LANG=ja
    if "%lang_choice%"=="10" set SELECTED_LANG=ko
    if "%lang_choice%"=="11" set SELECTED_LANG=ar
    if "%lang_choice%"=="12" set SELECTED_LANG=hi

    if "%SELECTED_LANG%"=="" (
        echo ❌ Invalid selection. Please choose 1-12.
        goto lang_menu
    )

    REM Command selection
    :cmd_menu
    echo.
    echo 🔧 Choose Command / Выберите команду:
    echo 1. 🚀 Build - Собрать сайт
    echo 2. 🧹 Clear - Очистить артефакты
    echo 3. 📊 Content - Статистика контента
    echo 4. ❓ Help - Справка
    echo 5. ℹ️  Version - Версия
    echo.
    set /p cmd_choice="Enter command number (1-5): "

    if "%cmd_choice%"=="1" set SELECTED_COMMAND=build
    if "%cmd_choice%"=="2" set SELECTED_COMMAND=clear
    if "%cmd_choice%"=="3" set SELECTED_COMMAND=content
    if "%cmd_choice%"=="4" set SELECTED_COMMAND=help
    if "%cmd_choice%"=="5" set SELECTED_COMMAND=version

    if "%SELECTED_COMMAND%"=="" (
        echo ❌ Invalid selection. Please choose 1-5.
        goto cmd_menu
    )

    REM Options for build command
    if "%SELECTED_COMMAND%"=="build" (
        echo.
        echo ⚙️  Choose Options / Выберите опции:
        echo 1. 🔇 Quiet mode - Тихий режим
        echo 2. 🎨 Choose theme - Выбрать тему
        echo 3. ✅ Confirm - Подтвердить
        echo.
        set /p option_choice="Enter option number (1-3): "

        if "%option_choice%"=="1" set QUIET_MODE=1
        if "%option_choice%"=="2" (
            echo.
            echo 🎨 Choose Theme / Выберите тему:
            echo 1. Default - Стандартная (с неоновой сеткой)
            echo 2. Dark - Темная
            echo 3. Light - Светлая
            echo.
            set /p theme_choice="Enter theme number (1-3): "
            if "%theme_choice%"=="1" set SELECTED_THEME=default
            if "%theme_choice%"=="2" set SELECTED_THEME=dark
            if "%theme_choice%"=="3" set SELECTED_THEME=light
            if "%SELECTED_THEME%"=="" set SELECTED_THEME=default
        )
    )

    echo.
    echo ℹ️  Configuration / Конфигурация:
    echo   Language: %SELECTED_LANG%
    echo   Command: %SELECTED_COMMAND%
    if "%SELECTED_COMMAND%"=="build" (
        echo   Quiet mode: %QUIET_MODE%
        if defined SELECTED_THEME echo   Theme: %SELECTED_THEME%
    )
    echo.
)

REM Check if Rust is available
cargo --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Error: Rust/Cargo is not installed.
    echo Please install Rust first: https://rustup.rs/
    pause
    exit /b 1
)

REM Build the Rust binary if needed
if not exist "src\bin\target\release\nw.exe" (
    echo 📦 Compiling NervaWeb...
    cargo build --release --manifest-path src\bin\Cargo.toml
    if errorlevel 1 (
        echo ❌ Error: Failed to compile NervaWeb.
        pause
        exit /b 1
    )
)

REM Copy the binary to project root if not already there
if not exist "nw.exe" (
    copy src\bin\target\release\nw.exe .\ >nul 2>&1
)

REM Build command arguments
set CMD_ARGS=%SELECTED_COMMAND%

if defined SELECTED_LANG if "%SELECTED_COMMAND%"=="build" (
    set CMD_ARGS=%CMD_ARGS% --lang %SELECTED_LANG%
)

if defined SELECTED_THEME if "%SELECTED_COMMAND%"=="build" (
    set CMD_ARGS=%CMD_ARGS% --theme %SELECTED_THEME%
)

if %QUIET_MODE%==1 (
    set CMD_ARGS=%CMD_ARGS% --quiet
)

REM Execute the command
if %QUIET_MODE%==0 (
    echo 🚀 Executing: nw %CMD_ARGS%
)

nw.exe %CMD_ARGS%

if errorlevel 1 (
    echo ❌ Command failed.
    pause
    exit /b 1
) else (
    if %QUIET_MODE%==0 (
        echo ✅ Operation completed successfully!
    )
)

if %QUIET_MODE%==0 (
    pause
)
:end
