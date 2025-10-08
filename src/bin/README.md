# NervaWeb Build Scripts

This directory contains build and deployment scripts for NervaWeb static site generator.

## Available Scripts

### Build Scripts

- **`build.bat`** - Windows build script with interactive menu
- **`build.sh`** - Linux/macOS build script with interactive menu
- **`build-arm.sh`** - ARM Linux build script (Raspberry Pi, etc.)

### Utility Scripts

- **`clean.bat`** - Windows cleanup script
- **`clean.sh`** - Linux/macOS cleanup script

### Deployment Scripts

- **`deploy.bat`** - Windows deployment script (copies to root for GitHub Pages)
- **`deploy.sh`** - Linux/macOS deployment script

## Interactive Menu System

All build scripts now include an interactive menu system for easy configuration:

### Language Selection
```
🌐 Choose Language / Выберите язык:
1. 🇷🇺 Русский (ru) - Russian
2. 🇺🇸 English (en) - English
3. 🇪🇸 Español (es) - Spanish
4. 🇩🇪 Deutsch (de) - German
5. 🇫🇷 Français (fr) - French
6. 🇮🇹 Italiano (it) - Italian
7. 🇵🇹 Português (pt) - Portuguese
8. 🇨🇳 中文 (zh) - Chinese
9. 🇯🇵 日本語 (ja) - Japanese
10. 🇰🇷 한국어 (ko) - Korean
11. 🇸🇦 العربية (ar) - Arabic
12. 🇮🇳 हिन्दी (hi) - Hindi
```

### Command Selection
```
🔧 Choose Command / Выберите команду:
1. 🚀 Build - Собрать сайт
2. 🧹 Clear - Очистить артефакты
3. 📊 Content - Статистика контента
4. ❓ Help - Справка
5. ℹ️  Version - Версия
```

### Options
```
⚙️  Choose Options / Выберите опции:
1. 🔇 Quiet mode - Тихий режим
2. 🎨 Choose theme - Выбрать тему
3. ✅ Confirm - Подтвердить
```

## Usage

### Interactive Mode (Recommended)

```bash
# Windows
src\bin\build.bat

# Linux/macOS
./src/bin/build.sh
```

### Command Line Mode

```bash
# Windows
src\bin\build.bat --lang en --quiet

# Linux/macOS
./src/bin/build.sh --lang en --quiet
```

### Quiet Mode

For automation and CI/CD pipelines:

```bash
# Build in quiet mode
nervaweb build --quiet

# Count content files only
nervaweb content --quiet
```

## Requirements

- **Rust**: `cargo install --path src/bin` or use the provided scripts
- **No external dependencies**: Pure Rust implementation

## Script Features

### Automatic Compilation
- Scripts automatically compile the Rust binary if needed
- Cross-platform support (Windows, Linux, macOS, ARM)

### Error Handling
- Comprehensive error checking
- Clear error messages in multiple languages
- Automatic cleanup on failure

### Deployment
- Automatic copying of built site to project root
- GitHub Pages ready
- Clean deployment process

## Examples

### Basic Usage
```bash
# Interactive build with menu
./src/bin/build.sh

# Quick build with default settings
nervaweb build

# Build specific language
nervaweb build --lang es

# Build with custom theme
nervaweb build --theme dark

# Clean build artifacts
nervaweb clear

# Get content statistics
nervaweb content
```

### Advanced Usage
```bash
# Build Spanish version quietly
nervaweb build --lang es --quiet

# Build for production deployment
nervaweb build --lang en --theme light

# Check content before build
nervaweb content && nervaweb build
```

## Platform-Specific Notes

### Windows
- Use `src\bin\build.bat` for interactive mode
- Ensure execution policy allows script running
- Use PowerShell or cmd

### Linux/macOS
- Make scripts executable: `chmod +x src/bin/*.sh`
- Use `./src/bin/build.sh` for interactive mode
- Compatible with bash, zsh, fish shells

### ARM (Raspberry Pi)
- Use `src/bin/build-arm.sh` for ARM-specific builds
- Optimized for ARM architecture
- Same interactive menu system
