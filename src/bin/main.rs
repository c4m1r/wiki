use std::env;
use std::fs;
use std::path::Path;
use regex::Regex;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{NervaLogic, NervaConfig, CliArgs, Command};

#[derive(Debug, Serialize, Deserialize)]
struct WikiConfig {
    title: String,
    description: String,
    language: String,
    output_dir: String,
}

fn main() {
    let args = CliArgs::new(env::args().collect());

    // Create logic instance
    let logic = match NervaLogic::new() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("❌ Failed to initialize NervaWeb: {}", e);
            std::process::exit(1);
        }
    };

    match Command::from_str(&args.command) {
        Some(Command::New) => handle_new(&args, &logic),
        Some(Command::Build) => handle_build(&args, &logic),
        Some(Command::Clear) => handle_clear(&args, &logic),
        Some(Command::Content) => handle_content(&args, &logic),
        Some(Command::Help) => handle_help(&args, &logic),
        Some(Command::Version) => handle_version(&args, &logic),
        None => {
            eprintln!("❌ Unknown command: {}", args.command);
            handle_help(&args, &logic);
            std::process::exit(1);
        }
    }
}

fn handle_new(args: &CliArgs, logic: &NervaLogic) {
    let project_name = match &args.project {
        Some(name) => name,
        None => {
            eprintln!("❌ Project name is required for 'new' command");
            eprintln!("Usage: nw new <project-name> [--desc \"Description\"]");
            std::process::exit(1);
        }
    };

    if !args.quiet {
        println!("📁 Creating new project '{}'...", project_name);
    }

    match logic.create_project(project_name, args.description.as_deref(), args.theme.as_deref()) {
        Ok(()) => {
            if !args.quiet {
                println!("✅ Project '{}' created successfully!", project_name);
                println!("📂 Project location: {}", logic.get_project_path(project_name).display());
                println!("⚙️  Config file: {}", logic.get_project_config_path(project_name).display());
                println!("📝 Content directory: {}", logic.get_project_content_dir(project_name, &NervaConfig::default()).display());
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to create project '{}': {}", project_name, e);
            std::process::exit(1);
        }
    }
}

fn handle_build(args: &CliArgs, logic: &NervaLogic) {
    let projects = match &args.project {
        Some(project_name) => {
            // Build specific project
            match logic.get_projects() {
                Ok(projects) => {
                    if !projects.contains(project_name) {
                        eprintln!("❌ Project '{}' not found", project_name);
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to get projects: {}", e);
                    std::process::exit(1);
                }
            }
            vec![project_name.clone()]
        }
        None => {
            // Build all projects
            match logic.get_projects() {
                Ok(projects) => {
                    if projects.is_empty() {
                        if !args.quiet {
                            println!("ℹ️  No projects found. Create one with: nervaweb new <project-name>");
                        }
                        return;
                    }
                    projects
                }
                Err(e) => {
                    eprintln!("❌ Failed to get projects: {}", e);
                    std::process::exit(1);
                }
            }
        }
    };

    for project_name in projects {
        if !args.quiet {
            println!("🚀 Building project '{}'...", project_name);
        }

        // Load project configuration
        let config = match logic.load_project_config(&project_name) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("❌ Failed to load config for project '{}': {}", project_name, e);
                continue;
            }
        };

        let lang = args.language.as_ref().unwrap_or(&config.default_language);
        let theme = args.theme.as_ref().map(|s| s.as_str()).unwrap_or_else(|| {
            if config.themes.is_empty() {
                "hello-world"
            } else {
                config.themes[0].as_str()
            }
        });
        env::set_var("LANG", lang);

        // Get project paths
        let content_dir = logic.get_project_content_dir(&project_name, &config);
        let output_dir = logic.get_project_output_path(&project_name);

    // Create output directory
    if output_dir.exists() {
            fs::remove_dir_all(&output_dir).expect("Failed to remove output directory");
    }
        fs::create_dir_all(&output_dir).expect("Failed to create output directory");

        // Copy static assets from generator root
        copy_static_assets(&output_dir, &logic.generator_root, &config, theme);

    // Process content files
        process_content_files(&output_dir, &content_dir, &logic.generator_root, lang);

    // Generate HTML files
        generate_html_files(&output_dir, &content_dir, &logic.generator_root, lang, &config, theme);

        if !args.quiet {
            println!("✅ Project '{}' built successfully!", project_name);
            println!("📁 Output available in: {}", output_dir.display());
        }
    }

    if !args.quiet && args.project.is_none() {
        println!("🎉 All projects built successfully!");
        println!("📁 Check the 'good2go' directory for all built sites");
    }
}

fn handle_clear(args: &CliArgs, logic: &NervaLogic) {
    if !args.quiet {
        println!("🧹 Clearing build artifacts...");
    }

    // Clear the good2go directory
    if logic.output_dir.exists() {
        match fs::remove_dir_all(&logic.output_dir) {
            Ok(()) => {
                if !args.quiet {
                    println!("🗑️  Removed good2go directory");
                }
            }
            Err(e) => {
                eprintln!("⚠️  Failed to remove good2go directory: {}", e);
            }
        }
    }

    // Clear target directory and Cargo.lock from generator
    let dirs_to_clear = vec![
        "target".to_string(),
        "Cargo.lock".to_string(),
    ];

    for dir in dirs_to_clear {
        let path = logic.generator_root.join(&dir);
        if path.exists() {
            if path.is_dir() {
                fs::remove_dir_all(&path).ok();
                if !args.quiet {
                    println!("🗑️  Removed directory: {}", dir);
                }
            } else {
                fs::remove_file(&path).ok();
                if !args.quiet {
                    println!("🗑️  Removed file: {}", dir);
                }
            }
        }
    }

    if !args.quiet {
        println!("✅ Cleanup completed!");
    }
}

fn handle_content(args: &CliArgs, logic: &NervaLogic) {
    if !args.quiet {
        println!("📊 Project Statistics:");
        println!("📁 Projects directory: {}", logic.projects_dir.display());
    }

    match logic.get_projects() {
        Ok(projects) => {
            if projects.is_empty() {
                if !args.quiet {
                    println!("📭 No projects found");
                }
            } else {
                let mut total_files = 0;
                let mut total_markdown_files = 0;

                for project_name in &projects {
                    if let Ok(config) = logic.load_project_config(project_name) {
                        let content_dir = logic.get_project_content_dir(project_name, &config);
                        let mut project_files = 0;
                        let mut project_markdown_files = 0;

                        if content_dir.exists() {
                            count_files_recursive(&content_dir, &mut project_files, &mut project_markdown_files, false);
                        }

                        total_files += project_files;
                        total_markdown_files += project_markdown_files;

                        if args.quiet {
                            println!("/projects/{}", project_name);
                            println!("{}", config.name);
                            println!("{}", config.description);
                            println!();
                        } else {
                            println!("/projects/{}", project_name);
                            println!("  📄 Name: {}", config.name);
                            println!("  📝 Description: {}", config.description);
                            println!("  📊 Files: {} (.md: {})", project_files, project_markdown_files);
                            println!();
                        }
                    }
                }

                if !args.quiet {
                    println!("📈 Summary:");
                    println!("  📂 Total projects: {}", projects.len());
                    println!("  📄 Total .md files: {}", total_markdown_files);
                    println!("  📁 Total files: {}", total_files);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get projects: {}", e);
            std::process::exit(1);
        }
    }
}

fn count_files_recursive(dir: &Path, total_files: &mut i32, markdown_files: &mut i32, verbose: bool) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            *total_files += 1;

            if path.is_dir() {
                count_files_recursive(&path, total_files, markdown_files, verbose);
            } else if path.extension().map_or(false, |ext| ext == "md") {
                *markdown_files += 1;
                if verbose {
                    println!("  📄 {}", path.strip_prefix(dir.parent().unwrap().parent().unwrap().parent().unwrap()).unwrap_or(&path).display());
                }
            }
        }
    }
}

fn handle_help(args: &CliArgs, logic: &NervaLogic) {
    println!("🌐 NervaWeb - Multi-Project Static Site Generator");
    println!("Version: 1.0.0");
    println!();
    println!("USAGE:");
    println!("    nw <COMMAND> [PROJECT] [OPTIONS]");
    println!();
    println!("COMMANDS:");
    println!("    new       {}", Command::New.description());
    println!("    build     {}", Command::Build.description());
    println!("    clear     {}", Command::Clear.description());
    println!("    content   {}", Command::Content.description());
    println!("    help      {}", Command::Help.description());
    println!("    version   {}", Command::Version.description());
    println!();
    println!("OPTIONS:");
    println!("    -l, --lang <LANG>    Set language for build");
    println!("    -t, --theme <THEME>  Set theme for build");
    println!("    -d, --desc <DESC>    Set description for new project");
    println!("    -q, --quiet          Quiet mode (minimal output)");
    println!("    -h, --help           Show this help");
    println!();
    println!("EXAMPLES:");
    println!("    nw new my-site                 # Create new project");
    println!("    nw new blog --desc \"My Blog\" # Create project with description");
    println!("    nw build                       # Build all projects");
    println!("    nw build my-site               # Build specific project");
    println!("    nw build my-site --lang en     # Build project in English");
    println!("    nw clear                       # Clear all build artifacts");
    println!("    nw content                     # List all projects with stats");
    println!("    nw content -q                  # Quiet project list");
    println!();
    println!("PROJECT STRUCTURE:");
    println!("    nervaweb-generator/");
    println!("    ├── projects/                  # All projects");
    println!("    │   └── my-site/              # Individual project");
    println!("    │       ├── content/          # Markdown files");
    println!("    │       ├── config.toml       # Project config");
    println!("    │       └── ...");
    println!("    ├── good2go/                  # Built sites");
    println!("    │   └── my-site/              # Built project");
    println!("    └── nw.exe                    # Generator binary");
}

fn handle_version(args: &CliArgs, logic: &NervaLogic) {
    println!("NervaWeb v1.0.0");
    println!("Multi-Project Static Site Generator");
    println!("Built with Rust 🦀");
}

fn copy_static_assets(output_dir: &Path, generator_root: &Path, config: &NervaConfig, theme: &str) {
    println!("📂 Copying static assets...");

    // Create .nojekyll file to disable Jekyll processing on GitHub Pages
    let nojekyll_path = output_dir.join(".nojekyll");
    fs::write(&nojekyll_path, "").ok();
    println!("📄 Created .nojekyll file for GitHub Pages compatibility");

    let src_dir = generator_root.join("src");

    // Copy theme files (all available themes)
    let themes_dir = src_dir.join("themes");
    if themes_dir.exists() {
        copy_dir_recursive(&themes_dir.to_string_lossy(), &output_dir.join("themes"));
    }

    // Copy CSS files
    let css_dir = src_dir.join("css");
    if css_dir.exists() {
        copy_dir_recursive(&css_dir.to_string_lossy(), &output_dir.join("css"));
    }

    // Copy JS files
    let js_dir = src_dir.join("js");
    if js_dir.exists() {
        copy_dir_recursive(&js_dir.to_string_lossy(), &output_dir.join("js"));
    }

    // Copy fonts and favicon from selected theme
    let selected_theme_dir = themes_dir.join(theme);
    if selected_theme_dir.exists() {
        let fonts_src = selected_theme_dir.join("fonts");
        if fonts_src.exists() {
            copy_dir_recursive(&fonts_src.to_string_lossy(), &output_dir.join("fonts"));
        }

        // Copy favicon files
        let favicon_files = ["favicon.png", "favicon.svg"];
        for favicon in &favicon_files {
            let src_file = selected_theme_dir.join(favicon);
            if src_file.exists() {
                fs::copy(&src_file, output_dir.join(favicon)).ok();
            }
        }
    }
}

fn copy_dir_recursive(src: &str, dst: &Path) {
    let src_path = Path::new(src);
    if !src_path.exists() {
        return;
    }

    fs::create_dir_all(dst).ok();

    for entry in fs::read_dir(src_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap();

        if path.is_dir() {
            copy_dir_recursive(&path.to_string_lossy(), &dst.join(file_name));
        } else {
            fs::copy(&path, dst.join(file_name)).ok();
        }
    }
}

fn process_content_files(output_dir: &Path, content_dir: &Path, generator_root: &Path, _lang: &str) {
    if !content_dir.exists() {
        return;
    }

    // Copy content directory structure to output
    copy_dir_recursive(&content_dir.to_string_lossy(), &output_dir.join("content"));
}

fn generate_html_files(output_dir: &Path, content_dir: &Path, generator_root: &Path, lang: &str, config: &NervaConfig, theme: &str) {
    // Generate main index.html
    generate_main_page(output_dir, content_dir, generator_root, lang, config, theme);

    // Process all markdown files in content/
    process_directory(&content_dir.to_string_lossy(), output_dir, generator_root, lang, config, theme);
}

fn generate_main_page(output_dir: &Path, content_dir: &Path, generator_root: &Path, lang: &str, config: &NervaConfig, theme: &str) {
    let template_path = generator_root.join("src").join("themes").join(theme).join("index.hbs");
    let template_content = fs::read_to_string(&template_path)
        .expect("Failed to read template");

    // Use index.md as main content file
    let content_path = content_dir.join("index.md");
    let content = process_markdown_file(&content_path.to_string_lossy(), lang);

    // Extract title from content
    let title = extract_title(&content).unwrap_or_else(|| config.name.clone());

    let html = template_content
        .replace("{{title}}", &format!("{} - {}", title, config.name))
        .replace("{{content}}", &content)
        .replace("{{language-switcher}}", &generate_language_switcher(lang, config));

    fs::write(output_dir.join("index.html"), html)
        .expect("Failed to write index.html");
}

fn generate_language_switcher(current_lang: &str, config: &NervaConfig) -> String {
    let mut buttons = String::new();

    for (code, name, flag) in &config.get_enabled_languages_info() {
        let is_active = code == current_lang;
        let background = if is_active { "var(--sidebar-bg)" } else { "var(--bg)" };
        let checkmark = if is_active { " ✓" } else { "" };

        buttons.push_str(&format!(
            r#"<button onclick="switchLanguage('{}')" style="background: {}; color: var(--fg); border: 1px solid var(--fg); padding: 6px 12px; border-radius: 4px; cursor: pointer;">{} {}{}</button>"#,
            code, background, flag, name, checkmark
        ));
    }

    format!(r#"
    <div id="language-switcher" style="position: fixed; top: 20px; right: 20px; z-index: 1000; display: flex; gap: 8px; background: var(--bg); padding: 8px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); border: 1px solid var(--fg);">
        {}
    </div>
    <script>
    function switchLanguage(lang) {{
        const currentPath = window.location.pathname.replace(/\.html$/, '');
        window.location.href = currentPath + '.html?lang=' + lang;
    }}
    </script>"#, buttons)
}

fn process_directory(src_dir: &str, dst_dir: &Path, generator_root: &Path, lang: &str, config: &NervaConfig, theme: &str) {
    let src_path = Path::new(src_dir);

    for entry in fs::read_dir(src_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_string_lossy();
            let new_dst_dir = dst_dir.join(dir_name.as_ref());

            if !new_dst_dir.exists() {
                fs::create_dir_all(&new_dst_dir).ok();
            }

            process_directory(&path.to_string_lossy(), &new_dst_dir, generator_root, lang, config, theme);
        } else if path.extension().map_or(false, |ext| ext == "md") {
            let file_stem = path.file_stem().unwrap().to_string_lossy();
            let html_file = dst_dir.join(format!("{}.html", file_stem));

            if file_stem == "index" || file_stem == "hello-world" {
                // Special handling for index files
                let content = process_markdown_file(&path.to_string_lossy(), lang);
                let title = extract_title(&content).unwrap_or_else(|| config.name.clone());

                let template_path = generator_root.join("src").join("themes").join(theme).join("index.hbs");
                let template_content = fs::read_to_string(&template_path)
                    .expect("Failed to read template");

                let html = template_content
                    .replace("{{title}}", &format!("{} - {}", title, config.name))
                    .replace("{{content}}", &content)
                    .replace("{{language-switcher}}", &generate_language_switcher(lang, config));

                fs::write(html_file, html).ok();
            } else {
                // Regular markdown files
                let content = process_markdown_file(&path.to_string_lossy(), lang);
                let title = extract_title(&content).unwrap_or_else(|| file_stem.to_string());

                let template_path = generator_root.join("src").join("themes").join(theme).join("index.hbs");
                let template_content = fs::read_to_string(&template_path)
                    .expect("Failed to read template");

                let html = template_content
                    .replace("{{title}}", &format!("{} - {}", title, config.name))
                    .replace("{{content}}", &content);

                fs::write(html_file, html).ok();
            }
        }
    }
}

fn process_markdown_file(file_path: &str, lang: &str) -> String {
    let content = fs::read_to_string(file_path).unwrap_or_default();

    // Extract language-specific content
    let html = markdown_to_html(&content, lang);

    html
}

fn extract_title(content: &str) -> Option<String> {
    for line in content.lines() {
        if line.starts_with("# ") {
            return Some(line[2..].trim().to_string());
        }
    }
    None
}

fn markdown_to_html(content: &str, current_lang: &str) -> String {
    let mut html = String::new();
    let mut in_code_block = false;
    let mut in_list = false;

    for line in content.lines() {
        // Handle code blocks
        if line.trim().starts_with("```") {
            if in_code_block {
                html.push_str("</code></pre>\n");
                in_code_block = false;
            } else {
                html.push_str("<pre><code>");
                in_code_block = true;
            }
            continue;
        }

        if in_code_block {
            html.push_str(&format!("{}\n", line));
            continue;
        }

        // Handle language blocks
        if line.trim().starts_with("<!-- LANG: ") && line.trim().ends_with(" -->") {
            let lang_code = line.trim()[11..line.trim().len()-4].trim();
            if lang_code != current_lang {
                // Skip content for other languages
                continue;
            }
        }

        if line.trim() == "<!-- END_LANG -->" {
            continue;
        }

        // Handle headers
        if line.starts_with("# ") {
            let title = &line[2..];
            html.push_str(&format!("<h1>{}</h1>\n", title));
        } else if line.starts_with("## ") {
            let title = &line[3..];
            html.push_str(&format!("<h2>{}</h2>\n", title));
        } else if line.starts_with("### ") {
            let title = &line[4..];
            html.push_str(&format!("<h3>{}</h3>\n", title));
        }
        // Handle lists
        else if line.trim().starts_with("- ") {
            if !in_list {
                html.push_str("<ul>\n");
                in_list = true;
            }
            let item = &line[2..];
            html.push_str(&format!("<li>{}</li>\n", item));
        } else if line.trim().is_empty() && in_list {
            html.push_str("</ul>\n");
            in_list = false;
        }
        // Handle links
        else if line.contains("[") && line.contains("]") && line.contains("(") && line.contains(")") {
            let processed = process_links(line);
            html.push_str(&format!("{}\n", processed));
        }
        // Regular text
        else if line.trim().is_empty() {
            html.push_str("<br>\n");
        } else {
            html.push_str(&format!("{}\n", line));
        }
    }

    if in_list {
        html.push_str("</ul>\n");
    }

    html
}

fn process_links(line: &str) -> String {
    // Simple link processing - replace [text](url) with <a href="url">text</a>
    let link_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
    link_regex.replace_all(line, r#"<a href="$2">$1</a>"#).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_html() {
        let markdown = "# Hello\n\n- Item 1\n- Item 2\n\n[Link](url)";
        let html = markdown_to_html(markdown, "ru");

        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<ul>"));
        assert!(html.contains("<li>Item 1</li>"));
        assert!(html.contains("<a href=\"url\">Link</a>"));
    }

    #[test]
    fn test_language_filtering() {
        let markdown = r#"<!-- LANG: ru -->
Russian content
<!-- END_LANG -->

<!-- LANG: en -->
English content
<!-- END_LANG -->"#;

        let html_ru = markdown_to_html(markdown, "ru");
        let html_en = markdown_to_html(markdown, "en");

        assert!(html_ru.contains("Russian content"));
        assert!(!html_ru.contains("English content"));
        assert!(html_en.contains("English content"));
        assert!(!html_en.contains("Russian content"));
    }
}
