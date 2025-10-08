#!/bin/bash
# NervaWeb Interactive Build Script for Linux/macOS

set -e

# Navigate to project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$(dirname "$SCRIPT_DIR")")"
cd "$PROJECT_ROOT"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check command line arguments
QUIET_MODE=false
SELECTED_LANG=""
SELECTED_THEME=""
SELECTED_COMMAND=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --quiet|-q)
            QUIET_MODE=true
            shift
            ;;
        --lang|-l)
            SELECTED_LANG="$2"
            shift 2
            ;;
        --theme|-t)
            SELECTED_THEME="$2"
            shift 2
            ;;
        --help|-h)
            echo "NervaWeb Interactive Build Script"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -q, --quiet     Quiet mode"
            echo "  -l, --lang LANG Language selection"
            echo "  -t, --theme THEME Theme selection"
            echo "  -h, --help      Show this help"
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Function to show language selection menu
show_language_menu() {
    echo ""
    echo -e "${CYAN}🌐 Choose Language / Выберите язык:${NC}"
    echo "1. 🇷🇺 Русский (ru) - Russian"
    echo "2. 🇺🇸 English (en) - English"
    echo "3. 🇪🇸 Español (es) - Spanish"
    echo "4. 🇩🇪 Deutsch (de) - German"
    echo "5. 🇫🇷 Français (fr) - French"
    echo "6. 🇮🇹 Italiano (it) - Italian"
    echo "7. 🇵🇹 Português (pt) - Portuguese"
    echo "8. 🇨🇳 中文 (zh) - Chinese"
    echo "9. 🇯🇵 日本語 (ja) - Japanese"
    echo "10. 🇰🇷 한국어 (ko) - Korean"
    echo "11. 🇸🇦 العربية (ar) - Arabic"
    echo "12. 🇮🇳 हिन्दी (hi) - Hindi"
    echo ""
}

# Function to show command selection menu
show_command_menu() {
    echo ""
    echo -e "${PURPLE}🔧 Choose Command / Выберите команду:${NC}"
    echo "1. 🚀 Build - Собрать сайт"
    echo "2. 🧹 Clear - Очистить артефакты"
    echo "3. 📊 Content - Статистика контента"
    echo "4. ❓ Help - Справка"
    echo "5. ℹ️  Version - Версия"
    echo ""
}

# Function to show options menu
show_options_menu() {
    echo ""
    echo -e "${YELLOW}⚙️  Choose Options / Выберите опции:${NC}"
    echo "1. 🔇 Quiet mode - Тихий режим"
    echo "2. 🎨 Choose theme - Выбрать тему"
    echo "3. ✅ Confirm - Подтвердить"
    echo ""
}

# Function to get language from selection
get_language_from_selection() {
    case $1 in
        1) echo "ru" ;;
        2) echo "en" ;;
        3) echo "es" ;;
        4) echo "de" ;;
        5) echo "fr" ;;
        6) echo "it" ;;
        7) echo "pt" ;;
        8) echo "zh" ;;
        9) echo "ja" ;;
        10) echo "ko" ;;
        11) echo "ar" ;;
        12) echo "hi" ;;
        *) echo "" ;;
    esac
}

# Function to get command from selection
get_command_from_selection() {
    case $1 in
        1) echo "build" ;;
        2) echo "clear" ;;
        3) echo "content" ;;
        4) echo "help" ;;
        5) echo "version" ;;
        *) echo "" ;;
    esac
}

# Function to show theme selection
show_theme_menu() {
    echo ""
    echo -e "${CYAN}🎨 Choose Theme / Выберите тему:${NC}"
    echo "1. Default - Стандартная (с неоновой сеткой)"
    echo "2. Dark - Темная"
    echo "3. Light - Светлая"
    echo ""
}

# Interactive mode
if [ "$QUIET_MODE" = false ] && [ -z "$SELECTED_LANG" ] && [ -z "$SELECTED_COMMAND" ]; then
    echo -e "${GREEN}🚀 NervaWeb Interactive Builder${NC}"
    echo "=============================="

    # Language selection
    while [ -z "$SELECTED_LANG" ]; do
        show_language_menu
        read -p "Enter language number (1-12): " lang_choice
        SELECTED_LANG=$(get_language_from_selection $lang_choice)
        if [ -z "$SELECTED_LANG" ]; then
            print_error "Invalid selection. Please choose 1-12."
        fi
    done

    # Command selection
    while [ -z "$SELECTED_COMMAND" ]; do
        show_command_menu
        read -p "Enter command number (1-5): " cmd_choice
        SELECTED_COMMAND=$(get_command_from_selection $cmd_choice)
        if [ -z "$SELECTED_COMMAND" ]; then
            print_error "Invalid selection. Please choose 1-5."
        fi
    done

    # Options for build command
    if [ "$SELECTED_COMMAND" = "build" ]; then
        show_options_menu
        read -p "Enter option number (1-3): " option_choice

        case $option_choice in
            1)
                QUIET_MODE=true
                ;;
            2)
                show_theme_menu
                read -p "Enter theme number (1-3): " theme_choice
                case $theme_choice in
                    1) SELECTED_THEME="default" ;;
                    2) SELECTED_THEME="dark" ;;
                    3) SELECTED_THEME="light" ;;
                    *) SELECTED_THEME="default" ;;
                esac
                ;;
            3)
                # Just confirm, no additional options
                ;;
            *)
                print_warning "Invalid option, proceeding with defaults."
                ;;
        esac
    fi

    echo ""
    print_info "Configuration:"
    echo "  Language: $SELECTED_LANG"
    echo "  Command: $SELECTED_COMMAND"
    if [ "$SELECTED_COMMAND" = "build" ]; then
        echo "  Quiet mode: $QUIET_MODE"
        if [ -n "$SELECTED_THEME" ]; then
            echo "  Theme: $SELECTED_THEME"
        fi
    fi
    echo ""
fi

# Check if Rust is available
if ! command -v cargo &> /dev/null; then
    print_error "Rust/Cargo is not installed."
    echo "Please install Rust first: https://rustup.rs/"
    exit 1
fi

# Build the Rust binary if needed
if [ ! -f "src/bin/target/release/nw" ]; then
    print_info "Compiling NervaWeb..."
    cargo build --release --manifest-path src/bin/Cargo.toml

    if [ $? -ne 0 ]; then
        print_error "Failed to compile NervaWeb."
        exit 1
    fi
fi

# Copy the binary to project root if not already there
if [ ! -f "nw" ]; then
    cp src/bin/target/release/nw ./ 2>/dev/null || true
fi

# Build command arguments
CMD_ARGS="$SELECTED_COMMAND"

if [ -n "$SELECTED_LANG" ] && [ "$SELECTED_COMMAND" = "build" ]; then
    CMD_ARGS="$CMD_ARGS --lang $SELECTED_LANG"
fi

if [ -n "$SELECTED_THEME" ] && [ "$SELECTED_COMMAND" = "build" ]; then
    CMD_ARGS="$CMD_ARGS --theme $SELECTED_THEME"
fi

if [ "$QUIET_MODE" = true ]; then
    CMD_ARGS="$CMD_ARGS --quiet"
fi

# Execute the command
if [ "$QUIET_MODE" = false ]; then
    print_info "Executing: nw $CMD_ARGS"
fi

./nw $CMD_ARGS

if [ $? -ne 0 ]; then
    print_error "Command failed."
    exit 1
fi

if [ "$QUIET_MODE" = false ]; then
    print_success "Operation completed successfully!"
fi
