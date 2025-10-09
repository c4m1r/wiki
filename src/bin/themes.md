# NervaWeb Themes Module

This module provides theme management functionality for NervaWeb.

## Available Themes

### Hello World Theme
- **ID**: `hello-world`
- **Description**: Simple theme with demo content and gradient background
- **Features**:
  - Multilingual support
  - Responsive design
  - Demo content included

### Wiki Theme
- **ID**: `wiki`
- **Description**: Full-featured wiki theme with language switcher
- **Features**:
  - Multilingual support with language switcher
  - Responsive design
  - Syntax highlighting
  - Advanced styling

## Theme Structure

```
themes/
├── hello-world/
│   ├── index.hbs          # Main HTML template (required)
│   ├── default.md         # Default content template (optional)
│   ├── styles.css         # Custom styles (optional)
│   ├── script.js          # Custom JavaScript (optional)
│   └── favicon.png        # Favicon (optional)
```

## API

### ThemeRegistry

```rust
// Create theme registry
let registry = ThemeRegistry::new(&generator_root);

// Get theme by ID
if let Some(theme) = registry.get_theme("hello-world") {
    println!("Theme: {}", theme.name);
    println!("Description: {}", theme.description);
    println!("Features: {:?}", theme.features);
}

// Get theme template
let template = registry.get_theme_template("hello-world", "index.hbs")?;

// Get default content
let content = registry.get_theme_content("hello-world")?;
```

### Theme

```rust
// Check if theme has a feature
if theme.has_feature("multilingual") {
    // Theme supports multilingual
}

// Get path to file in theme
let favicon_path = theme.get_file_path("favicon.png");
```

## Adding a New Theme

1. Create folder in `src/themes/your-theme/`
2. Add `index.hbs` file (required)
3. Add other files as needed
4. Theme will be automatically discovered at runtime

## Template Variables

Themes can use the following Handlebars variables:

- `{{title}}` - Page title
- `{{content}}` - Main content (HTML)
- `{{language}}` - Current language code
- `{{path_to_root}}` - Relative path to site root
- `{{base_url}}` - Base URL from configuration
- `{{language-switcher}}` - Language switcher HTML (if enabled)

## Publishing Themes

For publishing themes on crates.io:

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
   authors = ["Your Name"]
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

## Compatibility

- Rust 1.70+
- Handlebars for templates
- Serde for serialization
