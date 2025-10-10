use std::path::Path;
use std::collections::HashMap;
use serde_json;
use handlebars::Handlebars;
use crate::logic::NervaConfig;

pub fn generate_widget_config_json(config: &NervaConfig) -> String {
    let widget_config = serde_json::json!({
        "enable_language_switcher": config.enable_language_switcher,
        "enable_theme_switcher": config.enable_theme_switcher,
        "enable_visitor_counter": config.enable_visitor_counter,
        "enable_ticker": config.enable_ticker,
        "enable_search": config.enable_search,
        "enabled_languages": config.enabled_languages
    });

    serde_json::to_string(&widget_config).unwrap_or_else(|_| "{}".to_string())
}

pub fn create_template_data(title: &str, content: &str, config: &NervaConfig, lang: &str, path_to_root: &str) -> serde_json::Map<String, serde_json::Value> {
    let widget_config_json = generate_widget_config_json(config);

    let mut data = serde_json::Map::new();
    data.insert("title".to_string(), serde_json::Value::String(title.to_string()));
    data.insert("content".to_string(), serde_json::Value::String(content.to_string()));
    data.insert("description".to_string(), serde_json::Value::String(config.description.clone()));
    data.insert("language".to_string(), serde_json::Value::String(lang.to_string()));
    data.insert("default_theme".to_string(), serde_json::Value::String("light".to_string()));
    data.insert("preferred_dark_theme".to_string(), serde_json::Value::String("dark".to_string()));
    data.insert("text_direction".to_string(), serde_json::Value::String("ltr".to_string()));
    data.insert("path_to_root".to_string(), serde_json::Value::String(path_to_root.to_string()));
    data.insert("is_print".to_string(), serde_json::Value::Bool(false));
    data.insert("base_url".to_string(), serde_json::Value::String("".to_string()));
    data.insert("print_enable".to_string(), serde_json::Value::Bool(true));
    data.insert("copy_fonts".to_string(), serde_json::Value::Bool(true));
    data.insert("mathjax_support".to_string(), serde_json::Value::Bool(false));
    data.insert("search_enabled".to_string(), serde_json::Value::Bool(config.enable_search));
    data.insert("git_repository_url".to_string(), serde_json::Value::String("".to_string()));
    data.insert("git_repository_icon".to_string(), serde_json::Value::String("github".to_string()));
    data.insert("git_repository_edit_url".to_string(), serde_json::Value::String("".to_string()));
    data.insert("live_reload_endpoint".to_string(), serde_json::Value::String("".to_string()));
    data.insert("google_analytics".to_string(), serde_json::Value::String("".to_string()));
    data.insert("playground_line_numbers".to_string(), serde_json::Value::Bool(false));
    data.insert("playground_copyable".to_string(), serde_json::Value::Bool(false));
    data.insert("playground_js".to_string(), serde_json::Value::Bool(false));
    data.insert("search_js".to_string(), serde_json::Value::Bool(config.enable_search));
    data.insert("additional_css".to_string(), serde_json::Value::Array(vec![]));
    data.insert("additional_js".to_string(), serde_json::Value::Array(vec![]));
    data.insert("language-switcher".to_string(), serde_json::Value::String(crate::language::generate_language_switcher(lang, config)));
    data.insert(" nervaweb-config ".to_string(), serde_json::Value::String(widget_config_json));
    data.insert("enable_pwa".to_string(), serde_json::Value::Bool(config.enable_pwa));
    data.insert("enable_i18n".to_string(), serde_json::Value::Bool(config.enable_i18n));

    // Generate table of contents (empty for now)
    data.insert("toc".to_string(), serde_json::Value::String("".to_string()));

    // Generate navigation (empty for now)
    data.insert("previous".to_string(), serde_json::Value::Null);
    data.insert("next".to_string(), serde_json::Value::Null);

    data
}

pub fn render_template_with_data(template_path: &Path, data: &serde_json::Map<String, serde_json::Value>) -> String {
    let template_content = std::fs::read_to_string(template_path)
        .expect("Failed to read template");

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("page", &template_content)
        .expect("Failed to register template");

    handlebars.render("page", data)
        .expect("Failed to render template")
}
