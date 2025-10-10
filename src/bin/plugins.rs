use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::logic::NervaConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub plugin_type: PluginType,
    pub entry_point: String,
    pub dependencies: Vec<String>,
    pub config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginType {
    Theme,
    Widget,
    Processor,
    Extension,
}

#[derive(Debug, Clone)]
pub struct PluginRegistry {
    pub plugins: HashMap<String, PluginManifest>,
    pub plugin_dir: PathBuf,
}

impl PluginRegistry {
    pub fn new(generator_root: &Path) -> Self {
        let plugin_dir = generator_root.join("plugins");
        let mut registry = PluginRegistry {
            plugins: HashMap::new(),
            plugin_dir,
        };
        registry.load_plugins();
        registry
    }

    pub fn load_plugins(&mut self) {
        if !self.plugin_dir.exists() {
            if let Err(e) = fs::create_dir_all(&self.plugin_dir) {
                eprintln!("⚠️  Failed to create plugins directory: {}", e);
                return;
            }
        }

        if let Ok(entries) = fs::read_dir(&self.plugin_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let plugin_path = entry.path();
                    if plugin_path.is_dir() {
                        self.load_plugin_from_dir(&plugin_path);
                    }
                }
            }
        }
    }

    fn load_plugin_from_dir(&mut self, plugin_dir: &Path) {
        let manifest_path = plugin_dir.join("plugin.toml");

        if let Ok(content) = fs::read_to_string(&manifest_path) {
            match toml::from_str::<PluginManifest>(&content) {
                Ok(manifest) => {
                    let plugin_name = manifest.name.clone();
                    self.plugins.insert(plugin_name, manifest.clone());
                    println!("🔌 Loaded plugin: {}", manifest.name);
                }
                Err(e) => {
                    eprintln!("⚠️  Failed to parse plugin manifest {}: {}", manifest_path.display(), e);
                }
            }
        }
    }

    pub fn get_plugin(&self, name: &str) -> Option<&PluginManifest> {
        self.plugins.get(name)
    }

    pub fn get_plugins_by_type(&self, plugin_type: &PluginType) -> Vec<&PluginManifest> {
        self.plugins.values()
            .filter(|p| &p.plugin_type == plugin_type)
            .collect()
    }

    pub fn get_available_themes(&self) -> Vec<String> {
        let mut themes = vec!["hello-world".to_string(), "wiki".to_string(), "blog".to_string()];

        for plugin in self.get_plugins_by_type(&PluginType::Theme) {
            themes.push(plugin.name.clone());
        }

        themes
    }

    pub fn copy_plugin_assets(&self, output_dir: &Path, config: &NervaConfig) {
        for plugin in self.plugins.values() {
            let plugin_assets_dir = self.plugin_dir.join(&plugin.name).join("assets");

            if plugin_assets_dir.exists() {
                match plugin.plugin_type {
                    PluginType::Theme => {
                        if config.themes.contains(&plugin.name) {
                            copy_plugin_theme_assets(&plugin_assets_dir, output_dir);
                        }
                    }
                    PluginType::Widget | PluginType::Extension => {
                        copy_plugin_widget_assets(&plugin_assets_dir, output_dir, &plugin.name);
                    }
                    PluginType::Processor => {
                        // Processors don't need asset copying
                    }
                }
            }
        }
    }

    pub fn inject_plugin_scripts(&self, config: &NervaConfig) -> String {
        let mut scripts = String::new();

        for plugin in self.plugins.values() {
            match plugin.plugin_type {
                PluginType::Theme => {
                    if config.themes.contains(&plugin.name) {
                        scripts.push_str(&format!(
                            "<script src=\"{{{{ path_to_root }}}}{}.js\"></script>\n",
                            plugin.name
                        ));
                    }
                }
                PluginType::Widget | PluginType::Extension => {
                    scripts.push_str(&format!(
                        "<script src=\"{{{{ path_to_root }}}}plugins/{}.js\"></script>\n",
                        plugin.name
                    ));
                }
                PluginType::Processor => {}
            }
        }

        scripts
    }
}

fn copy_plugin_theme_assets(plugin_assets_dir: &Path, output_dir: &Path) {
    use std::fs;

    if let Ok(entries) = fs::read_dir(plugin_assets_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let src_path = entry.path();
                let file_name = src_path.file_name().unwrap();
                let dest_path = output_dir.join(file_name);

                if let Err(e) = fs::copy(&src_path, &dest_path) {
                    eprintln!("⚠️  Failed to copy plugin asset {}: {}", src_path.display(), e);
                }
            }
        }
    }
}

fn copy_plugin_widget_assets(plugin_assets_dir: &Path, output_dir: &Path, plugin_name: &str) {
    use std::fs;

    let plugins_output_dir = output_dir.join("plugins");
    if !plugins_output_dir.exists() {
        if let Err(e) = fs::create_dir_all(&plugins_output_dir) {
            eprintln!("⚠️  Failed to create plugins output directory: {}", e);
            return;
        }
    }

    if let Ok(entries) = fs::read_dir(plugin_assets_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let src_path = entry.path();
                let file_name = src_path.file_name().unwrap();
                let dest_path = plugins_output_dir.join(format!("{}_{}", plugin_name, file_name.to_string_lossy()));

                if let Err(e) = fs::copy(&src_path, &dest_path) {
                    eprintln!("⚠️  Failed to copy plugin asset {}: {}", src_path.display(), e);
                }
            }
        }
    }
}

pub fn create_example_plugin(name: &str, plugin_type: &PluginType) -> Result<(), Box<dyn std::error::Error>> {
    let plugin_dir = Path::new("plugins").join(name);

    if plugin_dir.exists() {
        return Err(format!("Plugin '{}' already exists", name).into());
    }

    fs::create_dir_all(&plugin_dir)?;

    // Create plugin.toml manifest
    let manifest = PluginManifest {
        name: name.to_string(),
        version: "1.0.0".to_string(),
        description: format!("Example {} plugin", match plugin_type {
            PluginType::Theme => "theme",
            PluginType::Widget => "widget",
            PluginType::Processor => "processor",
            PluginType::Extension => "extension",
        }),
        author: "NervaWeb User".to_string(),
        plugin_type: plugin_type.clone(),
        entry_point: format!("{}.js", name),
        dependencies: vec![],
        config: HashMap::new(),
    };

    let manifest_content = toml::to_string_pretty(&manifest)?;
    fs::write(plugin_dir.join("plugin.toml"), manifest_content)?;

    // Create assets directory
    let assets_dir = plugin_dir.join("assets");
    fs::create_dir_all(&assets_dir)?;

    // Create example JavaScript file
    let js_content = match plugin_type {
        PluginType::Theme => format!(r#"// NervaWeb Theme Plugin: {}
(function() {{
    'use strict';

    // Theme initialization
    document.addEventListener('DOMContentLoaded', function() {{
        console.log('Theme {} loaded');

        // Add custom theme styles
        const style = document.createElement('style');
        style.textContent = `
            /* Custom theme styles for {} */
            .theme-{{
                --custom-primary: #ff6b6b;
                --custom-secondary: #4ecdc4;
            }}
        `;
        document.head.appendChild(style);
    }});

    // Register theme
    if (window.NervaWeb) {{
        window.NervaWeb.themes = window.NervaWeb.themes || {{}};
        window.NervaWeb.themes['{}'] = {{
            name: '{}',
            version: '1.0.0',
            init: function() {{
                console.log('Initializing theme {}');
            }}
        }};
    }}
}})();
"#, name, name, name, name, name, name),
        PluginType::Widget => format!(r#"// NervaWeb Widget Plugin: {}
(function() {{
    'use strict';

    // Widget initialization
    document.addEventListener('DOMContentLoaded', function() {{
        console.log('Widget {} loaded');

        // Create widget container
        const widget = document.createElement('div');
        widget.id = '{}-widget';
        widget.className = 'nervaweb-plugin-widget';
        widget.innerHTML = `
            <div class="widget-header">
                <h3>{}</h3>
            </div>
            <div class="widget-content">
                <p>This is a custom widget from plugin {}</p>
                <button onclick="alert('Hello from {}!')">Click me</button>
            </div>
        `;

        // Add widget styles
        const style = document.createElement('style');
        style.textContent = `
            #{}-widget {{
                position: fixed;
                top: 100px;
                right: 10px;
                width: 250px;
                background: var(--bg);
                border: 1px solid var(--border);
                border-radius: 8px;
                padding: 15px;
                z-index: 1000;
                box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            }}
            #{}-widget .widget-header {{
                margin-bottom: 10px;
            }}
            #{}-widget .widget-content button {{
                background: var(--links);
                color: white;
                border: none;
                padding: 5px 10px;
                border-radius: 4px;
                cursor: pointer;
            }}
        `;
        document.head.appendChild(style);
        document.body.appendChild(widget);
    }});

    // Register widget
    if (window.NervaWeb) {{
        window.NervaWeb.widgets = window.NervaWeb.widgets || {{}};
        window.NervaWeb.widgets['{}'] = {{
            name: '{}',
            version: '1.0.0',
            init: function() {{
                console.log('Initializing widget {}');
            }}
        }};
    }}
}})();
"#, name, name, name, name, name, name, name, name, name, name, name, name),
        PluginType::Extension => format!(r#"// NervaWeb Extension Plugin: {}
(function() {{
    'use strict';

    // Extension initialization
    document.addEventListener('DOMContentLoaded', function() {{
        console.log('Extension {} loaded');

        // Add custom functionality
        window.NervaWebExtension = {{
            name: '{}',
            version: '1.0.0',
            extendFunctionality: function() {{
                console.log('Extending NervaWeb with {}');

                // Example: Add custom keyboard shortcuts
                document.addEventListener('keydown', function(e) {{
                    if (e.ctrlKey && e.key === 'e') {{
                        e.preventDefault();
                        alert('Custom shortcut from {} extension!');
                    }}
                }});
            }}
        }};

        // Initialize extension
        window.NervaWebExtension.extendFunctionality();
    }});

    // Register extension
    if (window.NervaWeb) {{
        window.NervaWeb.extensions = window.NervaWeb.extensions || {{}};
        window.NervaWeb.extensions['{}'] = window.NervaWebExtension;
    }}
}})();
"#, name, name, name, name, name, name),
        PluginType::Processor => format!(r#"// NervaWeb Processor Plugin: {}
// This is a server-side plugin that runs during build time
// It would be implemented in Rust as a separate crate

console.log('Processor plugin {} loaded (client-side stub)');
"#, name, name),
    };

    fs::write(assets_dir.join(format!("{}.js", name)), js_content)?;

    // Create README
    let readme_content = format!(r#"# {} Plugin

This is an example {} plugin for NervaWeb.

## Installation

1. Place this plugin in the `plugins/{}` directory
2. Enable the plugin in your NervaWeb configuration

## Usage

This plugin provides {} functionality to extend NervaWeb.

## Configuration

Add to your `config.toml`:

```toml
[plugins]
{} = {{ enabled = true }}
```

## Development

To modify this plugin:
1. Edit `plugin.toml` for metadata
2. Edit `assets/{}.js` for functionality
3. Add additional assets in the `assets/` directory
"#,
        name,
        match plugin_type {
            PluginType::Theme => "theme",
            PluginType::Widget => "widget",
            PluginType::Processor => "processor",
            PluginType::Extension => "extension",
        },
        name,
        match plugin_type {
            PluginType::Theme => "custom theming",
            PluginType::Widget => "UI widgets",
            PluginType::Processor => "content processing",
            PluginType::Extension => "extended functionality",
        },
        name,
        name
    );

    fs::write(plugin_dir.join("README.md"), readme_content)?;

    println!("✅ Created example plugin '{}' in plugins/{}", name, name);
    Ok(())
}
