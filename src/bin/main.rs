use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod logic;
mod console;
mod language;
mod variables;

use logic::{NervaLogic, NervaConfig, CliArgs, Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    // If no arguments provided, start interactive mode
    if args.len() == 1 {
        console::start_interactive_mode();
        return;
    }

    let cli_args = CliArgs::new(args);

    // Create logic instance
    let logic = match NervaLogic::new() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("✗ Failed to initialize NervaWeb: {}", e);
            std::process::exit(1);
        }
    };

    match Command::from_str(&cli_args.command) {
        Some(Command::New) => handle_new(&cli_args, &logic),
        Some(Command::Build) => handle_build(&cli_args, &logic),
        Some(Command::Clear) => handle_clear(&cli_args, &logic),
        Some(Command::Content) => handle_content(&cli_args, &logic),
        Some(Command::Help) => handle_help(&cli_args, &logic),
        Some(Command::Version) => handle_version(&cli_args, &logic),
        _ => {
            eprintln!("✗ Unknown command: {}", cli_args.command);
            eprintln!("Use 'nervaweb help' for usage information");
            std::process::exit(1);
        }
    }
}

fn handle_new(args: &CliArgs, logic: &NervaLogic) {
    let project_name = match &args.project {
        Some(name) => name,
        None => {
            eprintln!("✗ Project name is required for 'new' command");
            eprintln!("Usage: nw new <project-name> [--desc \"Description\"]");
            std::process::exit(1);
        }
    };

    if !args.quiet {
        println!("⌂ Creating new project '{}'...", project_name);
    }

    match logic.create_project(project_name, args.description.as_deref(), args.theme.as_deref()) {
        Ok(()) => {
            if !args.quiet {
                println!("✔ Project '{}' created successfully!", project_name);
                println!("⌂ Project location: {}", logic.get_project_path(project_name).display());
                println!("⚙️  Config file: {}", logic.get_project_config_path(project_name).display());
                println!("✍ Content directory: {}", logic.get_project_content_dir(project_name, &NervaConfig::default()).display());
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to create project '{}': {}", project_name, e);
            std::process::exit(1);
        }
    }
}

fn handle_build(args: &CliArgs, logic: &NervaLogic) {
    let projects = if let Some(ref project_name) = args.project {
        vec![project_name.clone()]
    } else {
        match logic.get_projects() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("✗ Failed to get projects: {}", e);
                std::process::exit(1);
            }
        }
    };

    for project_name in projects {
        if !args.quiet {
            println!("▲ Building project '{}'...", project_name);
        }

        match logic.load_project_config(&project_name) {
            Ok(config) => {
                let output_dir = logic.get_project_output_path(&project_name);
                let content_dir = logic.get_project_content_dir(&project_name, &config);
                let generator_root = logic.generator_root.clone();

                // Copy static assets
                copy_static_assets(&output_dir, &generator_root, &config, &config.themes[0]);

                // Generate HTML files
                generate_html_files(&output_dir, &content_dir, &generator_root, &config.default_language, &config, &config.themes[0]);

                if !args.quiet {
                    println!("✔ Project '{}' built successfully!", project_name);
                    println!("⌂ Output available in: {}", output_dir.display());
                }
            }
            Err(e) => {
                eprintln!("✗ Failed to load config for project '{}': {}", project_name, e);
                continue;
            }
        }
    }

    if !args.quiet {
        println!("🎉 All projects built successfully!");
        println!("⌂ Check the 'good2go' directory for all built sites");
    }
}

fn handle_clear(_args: &CliArgs, logic: &NervaLogic) {
    println!("Clearing build cache...");
    // TODO: Implement cache clearing logic
    println!("✔ Build cache cleared");
}

fn handle_content(_args: &CliArgs, logic: &NervaLogic) {
    let projects = match logic.get_projects() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("✗ Failed to get projects: {}", e);
            return;
        }
    };

    println!("📊 Content Statistics:");
    println!("⌂ Projects directory: {}", logic.projects_dir.display());

    let mut total_projects = 0;
    let mut total_configs = 0;

    for project in projects {
        total_projects += 1;
        let config_path = logic.get_project_config_path(&project);
        if config_path.exists() {
            total_configs += 1;
        }

        println!("/projects/{}", project);

        let content_dir = logic.get_project_content_dir(&project, &NervaConfig::default());
        if content_dir.exists() {
            let mut total_files = 0;
            let mut markdown_files = 0;
            count_files_recursive(&content_dir, &mut total_files, &mut markdown_files, false);
            println!("  ✍ Content files: {} ({} .md)", total_files, markdown_files);
        } else {
            println!("  ✗ Content directory not found");
        }

        let output_dir = logic.get_project_output_path(&project);
        if output_dir.exists() {
            println!("  ▲ Build output: {}", output_dir.display());
        }
    }

    println!("\n📈 Summary:");
    println!("⌂ Total projects: {}", total_projects);
    println!("⚙️  Projects with config: {}", total_configs);
}

fn count_files_recursive(dir: &Path, total_files: &mut i32, markdown_files: &mut i32, verbose: bool) -> i32 {
    let mut local_total = 0;

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                local_total += 1;
                *total_files += 1;

                if path.is_dir() {
                    count_files_recursive(&path, total_files, markdown_files, verbose);
                } else if let Some(extension) = path.extension() {
                    if extension == "md" {
                        *markdown_files += 1;
                    }
                }
            }
        }
    }

    local_total
}

fn handle_help(_args: &CliArgs, _logic: &NervaLogic) {
    println!("NervaWeb - Multi-Project Static Site Generator");
    println!("");
    println!("USAGE:");
    println!("    nervaweb <COMMAND> [OPTIONS]");
    println!("");
    println!("COMMANDS:");
    println!("    new <project-name>    Create a new project");
    println!("    build [project-name]  Build project(s)");
    println!("    clear                 Clear build cache");
    println!("    content               Show project statistics");
    println!("    help                  Show this help message");
    println!("    version               Show version information");
    println!("");
    println!("OPTIONS:");
    println!("    --lang <language>     Set language for build");
    println!("    --theme <theme>       Set theme for new project");
    println!("    --desc <description>  Set description for new project");
    println!("    --quiet               Suppress output");
    println!("");
    println!("EXAMPLES:");
    println!("    nervaweb new my-site --theme blog --desc \"My awesome site\"");
    println!("    nervaweb build my-site --lang ru");
    println!("    nervaweb build        # Build all projects");
    println!("");
    println!("∞ For more information, visit: https://github.com/c4m1r/wiki");
}

fn handle_version(_args: &CliArgs, _logic: &NervaLogic) {
    println!("NervaWeb v1.0.0");
    println!("Multi-Project Static Site Generator");
    println!("Built with Rust 🦀");
}

fn copy_static_assets(output_dir: &Path, generator_root: &Path, config: &NervaConfig, theme: &str) {
    println!("⌂ Copying static assets...");

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

    // Copy FontAwesome
    let fontawesome_dir = src_dir.join("FontAwesome");
    if fontawesome_dir.exists() {
        copy_dir_recursive(&fontawesome_dir.to_string_lossy(), &output_dir.join("FontAwesome"));
    }

    // Copy assets from selected theme
    let selected_theme_dir = themes_dir.join(theme);
    if selected_theme_dir.exists() {
        // Copy CSS directory from theme
        let css_src = selected_theme_dir.join("css");
        if css_src.exists() {
            copy_dir_recursive(&css_src.to_string_lossy(), &output_dir.join("css"));
        }

        // Copy fonts from theme
        let fonts_src = selected_theme_dir.join("fonts");
        if fonts_src.exists() {
            copy_dir_recursive(&fonts_src.to_string_lossy(), &output_dir.join("fonts"));
        }

        // Copy favicon directory
        let favicon_src = selected_theme_dir.join("favicon");
        if favicon_src.exists() {
            copy_dir_recursive(&favicon_src.to_string_lossy(), &output_dir.join("favicon"));
        }

        // Copy theme-specific JS files
        let theme_js_files = [
            "elasticlunr.min.js", "mark.min.js", "searcher.js", "clipboard.min.js",
            "ace.js", "editor.js", "mode-rust.js", "theme-dawn.js", "theme-tomorrow_night.js"
        ];
        for file in &theme_js_files {
            let src_file = selected_theme_dir.join(file);
            if src_file.exists() {
                fs::copy(&src_file, output_dir.join(file)).ok();
            }
        }

        // Copy theme-specific CSS and JS files to root
        let theme_files = [
            "highlight.css", "highlight.js", "ayu-highlight.css", "tomorrow-night.css",
            "nervaweb.js", "favicon.png", "favicon.svg", "icon.png", "icon.svg"
        ];
        for file in &theme_files {
            let src_file = selected_theme_dir.join(file);
            if src_file.exists() {
                fs::copy(&src_file, output_dir.join(file)).ok();
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
    process_directory(&content_dir.to_string_lossy(), output_dir, generator_root, lang, config, theme, output_dir);
}

fn generate_blog_page(output_dir: &Path, content_dir: &Path, generator_root: &Path, lang: &str, config: &NervaConfig, theme: &str) {
    let template_path = generator_root.join("src").join("themes").join(theme).join("index.hbs");

    // Read blog posts
    let blog_posts = read_blog_posts(content_dir, lang);

    // Create template data
    let mut data = variables::create_template_data("Blog - NervaWeb", "Welcome to our blog system", config, lang, "./");

    // Add blog-specific data
    data.insert("blogs".to_string(), serde_json::Value::Array(
        config.blogs.iter().map(|blog| {
            serde_json::json!({
                "id": blog.id,
                "title": blog.title,
                "description": blog.description,
                "path": blog.path,
                "enabled": blog.enabled
            })
        }).collect()
    ));

    data.insert("blog_posts".to_string(), serde_json::Value::Array(
        blog_posts.into_iter().map(|post| {
            serde_json::json!({
                "title": post.title,
                "url": post.url,
                "date_iso": post.date_iso,
                "date_formatted": post.date_formatted,
                "excerpt": post.excerpt
            })
        }).collect()
    ));

    // Render template
    let html = variables::render_template_with_data(&template_path, &data);
    fs::write(output_dir.join("index.html"), html)
        .expect("Failed to write blog index.html");
}

#[derive(Debug)]
struct BlogPost {
    title: String,
    url: String,
    date_iso: String,
    date_formatted: String,
    excerpt: String,
}

fn read_blog_posts(content_dir: &Path, lang: &str) -> Vec<BlogPost> {
    let blog_dir = content_dir.join("blog");
    if !blog_dir.exists() {
        return vec![];
    }

    let mut posts = vec![];

    // Read all .md files from blog directory
    for entry in fs::read_dir(&blog_dir).unwrap_or_else(|_| fs::read_dir(".").unwrap()) {
        let entry = entry.unwrap_or_else(|_| panic!("Failed to read blog directory"));
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "md") {
            let file_name = path.file_stem().unwrap().to_string_lossy();
            let content = language::process_markdown_file(&path.to_string_lossy(), lang);
            let title = language::extract_title(&content).unwrap_or_else(|| file_name.to_string());

            // Use file creation time as date
            let metadata = fs::metadata(&path).unwrap_or_else(|_| panic!("Failed to read file metadata"));
            let created = metadata.created().unwrap_or_else(|_| std::time::SystemTime::now());
            let datetime: chrono::DateTime<chrono::Utc> = created.into();
            let date_iso = datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string();
            let date_formatted = datetime.format("%d.%m.%Y %H:%M").to_string();

            // Create excerpt (first 200 characters)
            let excerpt = if content.len() > 200 {
                content.chars().take(200).collect::<String>() + "..."
            } else {
                content.clone()
            };

            posts.push(BlogPost {
                title,
                url: format!("blog/{}.html", file_name),
                date_iso,
                date_formatted,
                excerpt,
            });
        }
    }

    // Sort by creation date (newest first)
    posts.sort_by(|a, b| b.date_iso.cmp(&a.date_iso));
    posts
}

fn generate_main_page(output_dir: &Path, content_dir: &Path, generator_root: &Path, lang: &str, config: &NervaConfig, theme: &str) {
    if theme == "blog" {
        generate_blog_page(output_dir, content_dir, generator_root, lang, config, theme);
        return;
    }
    let template_path = generator_root.join("src").join("themes").join(theme).join("index.hbs");

    // Use index.md as main content file
    let content_path = content_dir.join("index.md");
    let content = language::process_markdown_file(&content_path.to_string_lossy(), lang);

    // Extract title from content
    let title = language::extract_title(&content).unwrap_or_else(|| config.name.clone());
    let full_title = format!("{} - {}", title, config.name);

    // Prepare data for template rendering
    let data = variables::create_template_data(&full_title, &content, config, lang, "./");

    // Render template with data
    let html = variables::render_template_with_data(&template_path, &data);

    fs::write(output_dir.join("index.html"), html)
        .expect("Failed to write index.html");
}

fn process_directory(src_dir: &str, dst_dir: &Path, generator_root: &Path, lang: &str, config: &NervaConfig, theme: &str, output_dir: &Path) {
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

            process_directory(&path.to_string_lossy(), &new_dst_dir, generator_root, lang, config, theme, output_dir);
        } else if path.extension().map_or(false, |ext| ext == "md") {
            let file_stem = path.file_stem().unwrap().to_string_lossy();
            let html_file = dst_dir.join(format!("{}.html", file_stem));

            // Calculate relative path to root for this file
            let relative_path = dst_dir.strip_prefix(output_dir).unwrap_or(dst_dir);
            let path_to_root = "../".repeat(relative_path.components().count()) + "./";

            let content = language::process_markdown_file(&path.to_string_lossy(), lang);
            let title = language::extract_title(&content).unwrap_or_else(|| file_stem.to_string());
            let full_title = format!("{} - {}", title, config.name);

            let template_path = generator_root.join("src").join("themes").join(theme).join("index.hbs");
            let data = variables::create_template_data(&full_title, &content, config, lang, &path_to_root);
            let html = variables::render_template_with_data(&template_path, &data);

            fs::write(html_file, html).ok();
        }
    }
}