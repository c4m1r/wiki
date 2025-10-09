use std::fs;
use regex::Regex;
use crate::logic::NervaConfig;

pub fn generate_language_switcher(current_lang: &str, config: &NervaConfig) -> String {
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

pub fn process_markdown_file(file_path: &str, lang: &str) -> String {
    let content = fs::read_to_string(file_path).unwrap_or_default();

    // Extract language-specific content
    let html = markdown_to_html(&content, lang);

    html
}

pub fn extract_title(content: &str) -> Option<String> {
    // Look for the first heading in the content
    let heading_regex = Regex::new(r"^#\s+(.+)$").unwrap();
    for line in content.lines() {
        if let Some(captures) = heading_regex.captures(line.trim()) {
            if let Some(title) = captures.get(1) {
                return Some(title.as_str().to_string());
            }
        }
    }
    None
}

pub fn markdown_to_html(content: &str, current_lang: &str) -> String {
    let mut html = String::new();
    let mut in_lang_block = false;
    let mut current_block_lang = String::new();
    let mut block_content = String::new();

    let lang_start_regex = Regex::new(r"<!--\s*LANG:\s*(\w+)\s*-->").unwrap();
    let lang_end_regex = Regex::new(r"<!--\s*END_LANG\s*-->").unwrap();

    for line in content.lines() {
        // Check for language block start
        if let Some(captures) = lang_start_regex.captures(line) {
            if let Some(lang_match) = captures.get(1) {
                in_lang_block = true;
                current_block_lang = lang_match.as_str().to_string();
                block_content.clear();
                continue;
            }
        }

        // Check for language block end
        if lang_end_regex.is_match(line) {
            if in_lang_block && current_block_lang == current_lang {
                // Process the accumulated block content as markdown
                html.push_str(&process_markdown_block(&block_content));
            }
            in_lang_block = false;
            current_block_lang.clear();
            block_content.clear();
            continue;
        }

        // If we're in a language block, accumulate content
        if in_lang_block {
            block_content.push_str(line);
            block_content.push('\n');
            continue;
        }

        // If we're not in a language block, process the line as regular markdown
        // For now, just pass through (we'll implement full markdown processing later)
        html.push_str(line);
        html.push('\n');
    }

    html
}

fn process_markdown_block(content: &str) -> String {
    let mut html = String::new();

    for line in content.lines() {
        // Simple heading processing
        if line.starts_with("# ") {
            html.push_str(&format!("<h1>{}</h1>\n", &line[2..]));
        } else if line.starts_with("## ") {
            html.push_str(&format!("<h2>{}</h2>\n", &line[3..]));
        } else if line.starts_with("### ") {
            html.push_str(&format!("<h3>{}</h3>\n", &line[4..]));
        } else if line.trim().is_empty() {
            html.push_str("<p></p>\n");
        } else {
            // Wrap non-empty lines in paragraphs
            if !line.trim().is_empty() {
                html.push_str(&format!("<p>{}</p>\n", line));
            }
        }
    }

    html
}

pub fn process_links(line: &str) -> String {
    let link_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
    link_regex.replace_all(line, r#"<a href="$2">$1</a>"#).to_string()
}
