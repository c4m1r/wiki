# NervaWeb

A multilingual static site generator written entirely in Rust. Create beautiful multilingual websites from Markdown files with GitHub/GitLab Pages and mirror support.

## ✪ What is NervaWeb?

**NervaWeb** is a modern static site generator specifically designed for creating multilingual projects.

### Key Features

1. **Pure Rust** - High performance and security
2. **Multilingual Support** - 12+ languages in a single file
3. **Deployment** - Built-in GitHub/GitLab Pages and mirror support
4. **Extensibility** - Easy to add new languages and themes
5. **User-Friendly** - Intuitive commands
6. **Flexibility** - Complete freedom in configuration

### Who is it for?

- Developers creating multilingual documentation
- Bloggers with international audiences
- Teams working on multi-language projects
- GitHub/GitLab Pages users for site deployment
- Anyone who wants a simple and powerful tool for generating sites with mirror support

## ▲ Quick Start

### Installing NervaWeb

```bash
# Global installation
cargo install --path src/bin

# Check installation
nervaweb --version
```

## ▲ Interactive Mode (Recommended for Beginners)

Run `nervaweb` without any arguments to launch the interactive mode:

```bash
# Launch interactive mode
nervaweb
```

**Features:**
- ♁ Beautiful ASCII animation with spider web and globe
- ∞ Automatic local server startup (ports 8000, 8001, etc.)
- ✪ 12-language menu system (English, Russian, Spanish, etc.)
- ✍ Complete project management
- ✿ Theme switching
- ✗ Cache management
- ⌨ Fallback to console mode

**Menu Options:**
1. ♁ Select Language (12 languages)
2. ∞ Publish Locally (all projects or selected)
3. + Create Project (with name, languages, theme)
4. ✗ Clear Good2Go Cache
5. ✗ Delete Project
6. ✗ Clear Cargo Cache
7. ⟲ Change Project Language
8. ✿ Change Project Theme
9. ⌨ Console Mode (professional mode)

### ⌨ Command Line Mode (Advanced Users)

```bash
# Create a new project
nervaweb new my-website --desc "My awesome website"

# Build the project
nervaweb build my-website

# View all projects
nervaweb content

# Clear builds
nervaweb clear
```

### Viewing Results

```bash
# Start local server
python3 -m http.server 8000 -d good2go/my-website
# or
npx serve good2go/my-website -p 8000
# or
simple-http-server good2go/my-website -p 8000
```

### Interactive Scripts

```bash
# Linux/macOS - interactive menu
./src/bin/build.sh

# Windows - interactive menu
src\bin\build.bat
```

### Deployment Configuration

After creating a project, open `projects/my-website/config.toml` and configure:

```toml
# Primary deployment URL
primary_deployment_url = "https://username.github.io/repository/"

# Additional mirrors for redundancy
[[deployment_mirrors]]
name = "GitHub Pages"
url = "https://username.github.io/repository/"
enabled = true

[[deployment_mirrors]]
name = "Local Development"
url = "http://localhost:8080/"
ip = "127.0.0.1"
port = 8080
enabled = false
```

### Deployment

1. **GitHub Pages**: Copy the contents of `good2go/my-website/` to the `docs/` folder or configure for the `gh-pages` branch
2. **GitLab Pages**: Copy the contents to the `public/` folder
3. **Other Hosting**: Upload the contents of `good2go/my-website/` to your server
4. **Mirrors**: Configure additional locations in `config.toml` for redundancy

### Example URLs for Different Platforms:

- **GitHub Pages**: `https://username.github.io/repository/`
- **GitLab Pages**: `https://username.gitlab.io/project/`
- **Netlify**: `https://amazing-site.netlify.app/`
- **Vercel**: `https://project-name.vercel.app/`
- **Custom Domain**: `https://yoursite.com/` or `https://blog.yoursite.com/`

### NervaWeb Architecture

```
⌂ nervaweb-workspace/          # NervaWeb workspace directory
├── ⌂ projects/               # All your projects
│   └── ⌂ my-website/         # Specific project
│       ├── ⌂ content/        # Markdown articles
│       │   ├── index.md       # Main page (created automatically)
│       │   └── about.md       # Other pages
│       └── config.toml        # Project configuration
├── ⌂ good2go/                # Built sites (ready for deployment)
│   └── ⌂ my-website/         # Ready HTML site
│       ├── index.html
│       ├── content/
│       ├── css/
│       ├── js/
│       ├── themes/            # All available themes
│       └── fonts/
├── ⌂ src/                    # Generator source code
│   ├── ⌂ themes/             # Themes with templates and content
│   │   ├── ⌂ hello-world/    # Simple theme (default)
│   │   │   ├── index.hbs      # HTML template
│   │   │   ├── default.md     # Content template for new projects
│   │   │   └── fonts/, css/   # Styles and fonts
│   │   └── ⌂ wiki/          # Full-featured theme
│   ├── ⌂ css/               # Global styles
│   ├── ⌂ js/                # JavaScript files
│   │   ├── nervaweb.js       # Unified JS library
│   │   ├── README.md          # JS documentation
│   │   └── [source files]     # Individual modules
│   └── ⌂ bin/               # Rust code
│       ├── main.rs           # Main logic
│       ├── logic.rs          # General project logic
│       ├── themes.rs         # Theme management module
│       ├── themes.md         # Theme documentation
│       ├── build.sh          # Linux/macOS interactive script
│       └── build.bat         # Windows interactive script
├── target/                    # Compiled binaries (new location)
├── nervaweb.exe               # Generator executable
└── README.md
```

### Features

- ♁ **Multilingual Support** - 12+ languages in one file
- ✿ **Custom Themes** - Complete freedom in design
- ⟲ **Dynamic Language Switching** - Without page reload
- ☎ **Responsive Design** - Works on all devices
- ⚡ **Pure Rust** - High performance and reliability
- ⌂ **Extensible Architecture** - Easy to add new features

## ℬ Content

- [Main Page](./src/content/index.md) - overview of all categories
- [Download Tools](./src/content/toolkit.md) - useful programs
- [Updates](./src/content/updates.md) - latest changes
- [Comments](./src/content/comments.md) - comment systems
- [Hidden Pages](./src/content/hidden-pages.md) - hidden content demonstration

## 🛠️ NervaWeb Commands

### Basic Commands

```bash
nervaweb new <name>         # Create new project
nervaweb build [project]    # Build project(s)
nervaweb clear              # Clear all builds
nervaweb content            # Show all projects with statistics
nervaweb help               # Show help
nervaweb version            # Show version
```

### Creating Projects

```bash
nervaweb new my-blog                    # Create project named my-blog
nervaweb new portfolio --desc "My Work" # Create with description
nervaweb new site --theme wiki          # Create with selected theme
nervaweb new blog --desc "My Blog" --theme hello-world # With description and theme
```

### Building Projects

```bash
nervaweb build                 # Build all projects
nervaweb build my-blog         # Build specific project
nervaweb build my-blog --lang en --theme wiki --quiet  # With English language, wiki theme, quiet mode
```

### Command Options

```bash
nervaweb new <name> [options]
  --desc, -d <DESC>     # Project description
  --theme, -t <THEME>   # Content theme (hello-world, wiki) - default hello-world

nervaweb build [project] [options]
  --lang, -l <LANG>     # Build language (ru, en, es, de, fr, it, pt, zh, ja, ko, ar, hi)
  --theme, -t <THEME>   # Theme (hello-world, wiki) - default hello-world
  --quiet, -q           # Quiet mode

nervaweb content [options]
  --quiet, -q           # File count only
```

### Usage Examples

```bash
# Creating projects
nervaweb new blog --desc "My personal blog"
nervaweb new docs --desc "Project documentation"

# Building
nervaweb build                    # All projects
nervaweb build blog               # Blog only
nervaweb build docs --lang en     # Documentation in English
nervaweb build blog --theme wiki  # Blog with wiki theme

# Management
nervaweb content                  # View all projects
nervaweb clear                    # Clear all builds

# Interactive scripts
./src/bin/build.sh          # Menu for Linux/macOS
src\bin\build.bat           # Menu for Windows
```

## 🛠️ Technical Details

**NervaWeb** is a completely original static site generator written in pure Rust without using third-party site generation libraries.

### Multilingual System

Articles are stored in unified files with blocks for each language:

```markdown
---
title:
  ru: Добро пожаловать
  en: Welcome
  es: Bienvenido
---

<!-- LANG: ru -->
Russian content here...
<!-- END_LANG -->

<!-- LANG: en -->
English content here...
<!-- END_LANG -->

<!-- LANG: es -->
Contenido español aquí...
<!-- END_LANG -->
```

### Approach Advantages

- ✔ **Unified Content Management** - One file for all languages
- ✔ **No Duplication** - Changes automatically apply to all languages
- ✔ **Easy Language Addition** - Just add a new LANG block
- ✔ **Clear Structure** - Languages explicitly separated in code

### Architecture

```
src/bin/
├── main.rs           # Entry point, command processing
├── logic.rs          # Project and language configuration
├── themes.rs         # Theme management module
└── preprocessor.rs   # Markdown and multilingual processing
```

### Supported Languages

NervaWeb supports 12 languages out of the box:
- 🇷🇺 **Russian** (ru) - primary language
- 🇺🇸 **English** (en)
- 🇪🇸 **Español** (es)
- 🇩🇪 **Deutsch** (de)
- 🇫🇷 **Français** (fr)
- 🇮🇹 **Italiano** (it)
- 🇵🇹 **Português** (pt)
- 🇨🇳 **中文** (zh)
- 🇯🇵 **日本語** (ja)
- 🇰🇷 **한국어** (ko)
- 🇸🇦 **العربية** (ar)
- 🇮🇳 **हिन्दी** (hi)

### Themes

NervaWeb supports a theme system that completely customizes site appearance:

- **hello-world** (default) - Simple theme with gradient background, demonstrating all Markdown capabilities
- **wiki** - Full-featured theme with neon grid, language switcher, and advanced features

#### Theme Structure

```
themes/[theme-name]/
├── index.hbs          # HTML page template
├── default.md         # Content template (optional)
├── fonts/             # Fonts
├── css/               # Theme styles
└── favicon.*          # Icons
```

#### Creating Custom Themes

1. Create folder `src/themes/my-theme/`
2. Add `index.hbs` with your HTML template
3. Add `default.md` with content template (optional)
4. Add theme to project `config.toml`
5. Build with `--theme my-theme`

**Note**: If a theme has `default.md`, it will be used as a template for new projects when selecting that theme.

### 📦 Publishing Themes via Crates.io

NervaWeb supports publishing themes as separate packages on [crates.io](https://crates.io):

1. **Create a separate crate** for your theme:
   ```bash
   cargo new nervaweb-theme-mytheme
   cd nervaweb-theme-mytheme
   ```

2. **Configure Cargo.toml**:
   ```toml
   [package]
   name = "nervaweb-theme-mytheme"
   version = "1.0.0"
   edition = "2021"
   description = "My awesome theme for NervaWeb"
   license = "MIT"
   repository = "https://github.com/username/nervaweb-theme-mytheme"
   keywords = ["nervaweb", "theme", "static-site"]
   categories = ["template-engine"]

   [dependencies]
   nervaweb-themes = "1.0"
   ```

3. **Organize file structure**:
   ```
   nervaweb-theme-mytheme/
   ├── Cargo.toml
   ├── src/
   │   └── lib.rs
   └── themes/
       └── mytheme/
           ├── index.hbs
           ├── default.md
           └── assets/
   ```

4. **Publish to crates.io**:
   ```bash
   cargo publish
   ```

5. **Use theme in projects**:
   ```toml
   [dependencies]
   nervaweb-theme-mytheme = "1.0"
   ```

This allows creating and distributing themes independently from the main NervaWeb generator.

## ❓ FAQ - Frequently Asked Questions

### 🔍 Where can I use the `nervaweb` command?

**The `nervaweb` command works ONLY inside the NervaWeb project folder where the proper file structure exists.**

#### ✔ Correct Usage:

```bash
# Navigate to NervaWeb project folder
cd /path/to/nervaweb-project

# Now all commands work
nervaweb build     # ✔ Build site
nervaweb clear     # ✔ Clear
nervaweb content   # ✔ Statistics

# From any other folder
cd /any/other/folder
nervaweb build     # ❌ ERROR - project structure not found
```

#### 🔧 How project detection works:

NervaWeb automatically determines the project root based on the executable location:

1. **If `nervaweb.exe` is in project root** (recommended):
   ```
   project/
   ├── nervaweb.exe          ← Executable here
   └── src/content/          ← Content searched relative to nervaweb.exe
   ```

2. **If `nervaweb.exe` is in `src/bin/target/release`** (during development):
   ```
   project/
   ├── src/
   │   └── bin/
   │       └── target/
   │           └── release/
   │               └── nervaweb.exe  ← Automatically goes up
   ```

#### ⌂ Required project structure:

```
nervaweb-project/
├── src/
│   ├── content/     # Articles (.md files)
│   │   ├── index.md
│   │   ├── glpi/
│   │   ├── story/
│   │   └── ...
│   ├── themes/      # HTML templates
│   │   └── index.hbs
│   ├── css/         # Styles
│   └── bin/         # Source code
├── good2go/        # Output folder (created by nervaweb build)
└── nervaweb.exe    # Executable (created by installation)
```

### ▲ How to install NervaWeb globally?

```bash
# Navigate to project folder
cd /path/to/nervaweb-project

# Install globally
cargo install --path src/bin

# Now nervaweb is available from any folder
nervaweb --version
```

### ♁ How to add a new language?

1. **Add language to `logic.rs`:**
```rust
enabled_languages: vec![
    "ru".to_string(),
    "en".to_string(),
    "es".to_string(),
    "new_lang".to_string(), // Your new language
],
```

2. **Add name and flag:**
```rust
pub fn get_language_name(&self, lang: &str) -> String {
    match lang {
        "new_lang" => "New Language".to_string(),
        // ... other languages
        _ => lang.to_string(),
    }
}
```

3. **Use in articles:**
```markdown
<!-- LANG: new_lang -->
Content in new language
<!-- END_LANG -->
```

### ✿ How to create a new theme?

1. **Create theme folder:**
```
src/themes/
├── hello-world/
├── wiki/
└── my-theme/          # New theme
    ├── index.hbs      # Template
    ├── css/           # Theme styles
    └── assets/        # Resources
```

2. **Add theme to `logic.rs`:**
```rust
themes: vec![
    "hello-world".to_string(),
    "wiki".to_string(),
    "my-theme".to_string(), // New theme
],
```

3. **Use:**
```bash
nervaweb build --theme my-theme
```

### ⚡ How to speed up builds?

```bash
# Quiet mode (minimal output)
nervaweb build --quiet

# Use for automation
nervaweb build --lang en --quiet
```

### 🔧 How to update NervaWeb?

```bash
# If installed globally
cargo install --path src/bin --force

# Or rebuild locally
cd src/bin
cargo build --release
cp target/release/nervaweb ../..
```

### 📊 What does the `content` command show?

```bash
nervaweb content        # Show all files with paths
nervaweb content -q     # File count only
```

Example output:
```
📊 Content Statistics:
⌂ Content directory: src/content
📄 Total .md files: 42
⌂ Total files: 156
```

### 🐛 What to do with errors?

**Problem:** `Failed to read template`
```
✔ Solution: Check that src/themes/index.hbs exists
```

**Problem:** `Language not supported`
```
✔ Solution: Add language to logic.rs or use supported one
```

**Problem:** `Command not found`
```
✔ Solution: Install NervaWeb: cargo install --path src/bin
```

### 🔄 Can it be used with CI/CD?

Yes! NervaWeb is perfect for automation:

```yaml
# .github/workflows/deploy.yml
name: Deploy Site
on: [push]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install --path src/bin
      - run: nervaweb build --lang en --quiet
      - run: nervaweb build --lang ru --quiet
```

### ⌨ Platform-Specific Installation and Usage

#### ♁ Windows

**Requirements:**
- Windows 10/11
- [Rust](https://rustup.rs/) installed
- Git (optional, for cloning repository)

**Installation:**
```bash
# Clone repository
git clone https://github.com/yourusername/nervaweb.git
cd nervaweb

# Build executable
cargo build --release --manifest-path src\bin\Cargo.toml

# Copy to project root (recommended)
copy src\bin\target\release\nervaweb.exe .\nervaweb.exe
```

**Usage:**
```bash
# Create project
nervaweb.exe new my-site --desc "My website"

# Build project
nervaweb.exe build my-site

# Start local server
python -m http.server 8000 -d good2go\my-site
```

**Interactive scripts:**
```cmd
# Run build script
src\bin\build.bat
```

#### ⚙ Linux

**Requirements:**
- Ubuntu/Debian/CentOS/RHEL/Fedora or other Linux distribution
- [Rust](https://rustup.rs/) installed
- Git (optional, for cloning repository)

**Installation:**
```bash
# Clone repository
git clone https://github.com/yourusername/nervaweb.git
cd nervaweb

# Build executable
cargo build --release --manifest-path src/bin/Cargo.toml

# Copy to project root (recommended)
cp src/bin/target/release/nervaweb ./nervaweb

# Make executable
chmod +x nervaweb
```

**Usage:**
```bash
# Create project
./nervaweb new my-site --desc "My website"

# Build project
./nervaweb build my-site

# Start local server
python3 -m http.server 8000 -d good2go/my-site
```

**Interactive scripts:**
```bash
# Run build script
./src/bin/build.sh
```

#### ▲ macOS

**Requirements:**
- macOS 10.15 or later
- [Xcode Command Line Tools](https://developer.apple.com/xcode/) installed
- [Rust](https://rustup.rs/) installed
- Git (optional, for cloning repository)

**Installation:**
```bash
# Install Xcode Command Line Tools (if not installed)
xcode-select --install

# Clone repository
git clone https://github.com/yourusername/nervaweb.git
cd nervaweb

# Build executable
cargo build --release --manifest-path src/bin/Cargo.toml

# Copy to project root (recommended)
cp src/bin/target/release/nervaweb ./nervaweb

# Make executable
chmod +x nervaweb
```

**Usage:**
```bash
# Create project
./nervaweb new my-site --desc "My website"

# Build project
./nervaweb build my-site

# Start local server
python3 -m http.server 8000 -d good2go/my-site
```

**Interactive scripts:**
```bash
# Run build script
./src/bin/build.sh
```

#### □ Docker (Cross-Platform)

**Requirements:**
- [Docker](https://docker.com/) installed

**Usage:**
```bash
# Clone repository
git clone https://github.com/yourusername/nervaweb.git
cd nervaweb

# Build Docker image
docker build -t nervaweb .

# Create project
docker run --rm -v $(pwd):/app nervaweb new my-site --desc "My website"

# Build project
docker run --rm -v $(pwd):/app nervaweb build my-site

# Start local server
docker run --rm -p 8000:8000 -v $(pwd)/good2go/my-site:/site \
  nginx:alpine -c /dev/null -p /site -l 8000
```

#### ☁ GitHub Codespaces / GitPod / VS Code Remote

**Automatic Setup:**
If you're using GitHub Codespaces or similar cloud development environment, Rust is usually pre-installed.

```bash
# Clone and build
git clone https://github.com/yourusername/nervaweb.git
cd nervaweb
cargo build --release --manifest-path src/bin/Cargo.toml
cp src/bin/target/release/nervaweb ./nervaweb
chmod +x nervaweb

# Use normally
./nervaweb new my-site
./nervaweb build my-site
```

#### 🔧 Global Installation (All Platforms)

**Using Cargo:**
```bash
# Install globally (works on any platform with Rust)
cargo install --path src/bin

# Now nervaweb is available system-wide
nervaweb --version
nervaweb new my-site
```

**Manual Installation:**
1. Download pre-built binary from [Releases](https://github.com/yourusername/nervaweb/releases)
2. Extract to a folder in your PATH
3. Make executable (Linux/macOS): `chmod +x nervaweb`

### ✪ Platform-Specific Notes

#### Windows
- Use `nervaweb.exe` instead of `nervaweb`
- Use `python` instead of `python3`
- Use backslashes `\` in paths within batch files
- Interactive scripts: `build.bat`, `deploy.bat`, `clean.bat`

#### Linux
- Use `./nervaweb` to run executable
- Use `python3` for local server
- Interactive scripts: `build.sh`, `deploy.sh`, `clean.sh`
- May need additional packages: `build-essential`, `pkg-config`

#### macOS
- Same as Linux, but Xcode Command Line Tools required
- Use `python3` from Homebrew if system Python is old
- May need to install additional tools with Homebrew

#### ARM Devices (Raspberry Pi, etc.)
```bash
# Use ARM build script
./src/bin/build-arm.sh

# Or cross-compile
rustup target add arm-unknown-linux-gnueabihf
cargo build --release --target arm-unknown-linux-gnueabihf --manifest-path src/bin/Cargo.toml
```

### 🔍 Troubleshooting by Platform

#### Windows Issues
```
Error: 'nervaweb' is not recognized
Solution: Use 'nervaweb.exe' or add to PATH
```

```
Error: cargo not found
Solution: Install Rust from https://rustup.rs/
```

#### Linux Issues
```
Error: Permission denied
Solution: chmod +x nervaweb
```

```
Error: libssl not found
Solution: sudo apt-get install libssl-dev pkg-config
```

#### macOS Issues
```
Error: xcode-select not found
Solution: xcode-select --install
```

```
Error: ld: library not found
Solution: Install Xcode Command Line Tools
```

### 💡 Usage Tips

1. **Always work in project folder** - `nervaweb` searches for structure relative to its location
2. **Use `--quiet` for scripts** - reduces log output
3. **Add languages gradually** - translate articles step by step
4. **Test locally** - use `python3 -m http.server 8000` for checking
5. **Make backups** - `nervaweb clear` removes all generated files

### 📞 Support

- 📖 **Documentation:** README.md
- 🐛 **Bugs:** Create issues in repository
- 💡 **Ideas:** Suggestions are welcome
- 🤝 **Contributions:** PRs always welcome!