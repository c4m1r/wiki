use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationData {
    pub languages: HashMap<String, LanguageData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageData {
    pub name: String,
    pub flag: String,
    pub strings: HashMap<String, String>,
}

impl Default for TranslationData {
    fn default() -> Self {
        let mut data = TranslationData {
            languages: HashMap::new(),
        };

        // English
        let mut en_strings = HashMap::new();
        en_strings.insert("search_placeholder".to_string(), "Search...".to_string());
        en_strings.insert("search_shortkey".to_string(), "Search. (Shortkey: s)".to_string());
        en_strings.insert("theme_change".to_string(), "Change theme".to_string());
        en_strings.insert("visitors".to_string(), "Visitors:".to_string());
        en_strings.insert("copy_tooltip".to_string(), "Copied!".to_string());
        en_strings.insert("copy_error".to_string(), "Copy failed!".to_string());
        en_strings.insert("no_results".to_string(), "No results found".to_string());
        en_strings.insert("search_error".to_string(), "Search failed to load index".to_string());
        en_strings.insert("print_site".to_string(), "Print this site".to_string());
        en_strings.insert("git_repository".to_string(), "Git repository".to_string());
        en_strings.insert("suggest_edit".to_string(), "Suggest an edit".to_string());
        en_strings.insert("table_of_contents".to_string(), "Toggle Table of Contents".to_string());
        en_strings.insert("search_toggle".to_string(), "Toggle Searchbar".to_string());

        data.languages.insert("en".to_string(), LanguageData {
            name: "English".to_string(),
            flag: "🇺🇸".to_string(),
            strings: en_strings,
        });

        // Russian
        let mut ru_strings = HashMap::new();
        ru_strings.insert("search_placeholder".to_string(), "Поиск...".to_string());
        ru_strings.insert("search_shortkey".to_string(), "Поиск. (Клавиша: s)".to_string());
        ru_strings.insert("theme_change".to_string(), "Сменить тему".to_string());
        ru_strings.insert("visitors".to_string(), "Посетители:".to_string());
        ru_strings.insert("copy_tooltip".to_string(), "Скопировано!".to_string());
        ru_strings.insert("copy_error".to_string(), "Ошибка копирования!".to_string());
        ru_strings.insert("no_results".to_string(), "Результаты не найдены".to_string());
        ru_strings.insert("search_error".to_string(), "Не удалось загрузить индекс поиска".to_string());
        ru_strings.insert("print_site".to_string(), "Распечатать сайт".to_string());
        ru_strings.insert("git_repository".to_string(), "Репозиторий Git".to_string());
        ru_strings.insert("suggest_edit".to_string(), "Предложить правку".to_string());
        ru_strings.insert("table_of_contents".to_string(), "Показать оглавление".to_string());
        ru_strings.insert("search_toggle".to_string(), "Показать поиск".to_string());

        data.languages.insert("ru".to_string(), LanguageData {
            name: "Русский".to_string(),
            flag: "🇷🇺".to_string(),
            strings: ru_strings,
        });

        // Spanish
        let mut es_strings = HashMap::new();
        es_strings.insert("search_placeholder".to_string(), "Buscar...".to_string());
        es_strings.insert("search_shortkey".to_string(), "Buscar. (Tecla: s)".to_string());
        es_strings.insert("theme_change".to_string(), "Cambiar tema".to_string());
        es_strings.insert("visitors".to_string(), "Visitantes:".to_string());
        es_strings.insert("copy_tooltip".to_string(), "¡Copiado!".to_string());
        es_strings.insert("copy_error".to_string(), "¡Error al copiar!".to_string());
        es_strings.insert("no_results".to_string(), "No se encontraron resultados".to_string());
        es_strings.insert("search_error".to_string(), "Error al cargar el índice de búsqueda".to_string());
        es_strings.insert("print_site".to_string(), "Imprimir sitio".to_string());
        es_strings.insert("git_repository".to_string(), "Repositorio Git".to_string());
        es_strings.insert("suggest_edit".to_string(), "Sugerir una edición".to_string());
        es_strings.insert("table_of_contents".to_string(), "Alternar tabla de contenidos".to_string());
        es_strings.insert("search_toggle".to_string(), "Alternar barra de búsqueda".to_string());

        data.languages.insert("es".to_string(), LanguageData {
            name: "Español".to_string(),
            flag: "🇪🇸".to_string(),
            strings: es_strings,
        });

        // German
        let mut de_strings = HashMap::new();
        de_strings.insert("search_placeholder".to_string(), "Suchen...".to_string());
        de_strings.insert("search_shortkey".to_string(), "Suchen. (Taste: s)".to_string());
        de_strings.insert("theme_change".to_string(), "Thema ändern".to_string());
        de_strings.insert("visitors".to_string(), "Besucher:".to_string());
        de_strings.insert("copy_tooltip".to_string(), "Kopiert!".to_string());
        de_strings.insert("copy_error".to_string(), "Kopieren fehlgeschlagen!".to_string());
        de_strings.insert("no_results".to_string(), "Keine Ergebnisse gefunden".to_string());
        de_strings.insert("search_error".to_string(), "Suchindex konnte nicht geladen werden".to_string());
        de_strings.insert("print_site".to_string(), "Seite drucken".to_string());
        de_strings.insert("git_repository".to_string(), "Git-Repository".to_string());
        de_strings.insert("suggest_edit".to_string(), "Bearbeitung vorschlagen".to_string());
        de_strings.insert("table_of_contents".to_string(), "Inhaltsverzeichnis umschalten".to_string());
        de_strings.insert("search_toggle".to_string(), "Suchleiste umschalten".to_string());

        data.languages.insert("de".to_string(), LanguageData {
            name: "Deutsch".to_string(),
            flag: "🇩🇪".to_string(),
            strings: de_strings,
        });

        data
    }
}

impl TranslationData {
    pub fn get_string(&self, lang: &str, key: &str) -> String {
        self.languages
            .get(lang)
            .and_then(|lang_data| lang_data.strings.get(key))
            .cloned()
            .unwrap_or_else(|| {
                // Fallback to English
                self.languages
                    .get("en")
                    .and_then(|lang_data| lang_data.strings.get(key))
                    .cloned()
                    .unwrap_or_else(|| key.to_string())
            })
    }

    pub fn get_available_languages(&self) -> Vec<(String, String, String)> {
        self.languages
            .iter()
            .map(|(code, data)| (code.clone(), data.name.clone(), data.flag.clone()))
            .collect()
    }
}

pub fn create_i18n_script(lang: &str, translations: &TranslationData) -> String {
    let mut script = format!(r#"
// NervaWeb Internationalization
window.NervaWeb = window.NervaWeb || {{}};
window.NervaWeb.i18n = {{
    currentLang: '{}',
    strings: {{
"#, lang);

    if let Some(lang_data) = translations.languages.get(lang) {
        for (key, value) in &lang_data.strings {
            script.push_str(&format!("        '{}': '{}',\n", key, value.replace("'", "\\'")));
        }
    }

    script.push_str(r#"    },
    t: function(key) {
        return this.strings[key] || key;
    },
    setLanguage: function(lang) {
        if (this.currentLang !== lang) {
            this.currentLang = lang;
            // Reload page with new language
            const url = new URL(window.location);
            url.searchParams.set('lang', lang);
            window.location.href = url.toString();
        }
    }
};

// Update UI elements with translations
document.addEventListener('DOMContentLoaded', function() {
    // Update placeholders
    const searchInput = document.getElementById('es-search-input');
    if (searchInput) {
        searchInput.placeholder = window.NervaWeb.i18n.t('search_placeholder');
    }

    // Update tooltips and titles
    const themeToggle = document.getElementById('theme-toggle');
    if (themeToggle) {
        themeToggle.title = window.NervaWeb.i18n.t('theme_change');
        themeToggle.setAttribute('aria-label', window.NervaWeb.i18n.t('theme_change'));
    }

    const searchToggle = document.querySelector('[aria-controls="searchbar"]');
    if (searchToggle) {
        searchToggle.title = window.NervaWeb.i18n.t('search_toggle');
        searchToggle.setAttribute('aria-label', window.NervaWeb.i18n.t('search_toggle'));
    }

    const tocToggle = document.querySelector('[aria-controls="sidebar"]');
    if (tocToggle) {
        tocToggle.title = window.NervaWeb.i18n.t('table_of_contents');
        tocToggle.setAttribute('aria-label', window.NervaWeb.i18n.t('table_of_contents'));
    }

    // Update print button
    const printButton = document.getElementById('print-button');
    if (printButton) {
        printButton.title = window.NervaWeb.i18n.t('print_site');
    }

    // Update git buttons
    const gitButton = document.getElementById('git-repository-button');
    if (gitButton) {
        gitButton.setAttribute('aria-label', window.NervaWeb.i18n.t('git_repository'));
    }

    const editButton = document.getElementById('git-edit-button');
    if (editButton) {
        editButton.setAttribute('aria-label', window.NervaWeb.i18n.t('suggest_edit'));
    }
});
"#);

    script
}
