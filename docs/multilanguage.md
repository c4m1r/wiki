# Multi-language Support

This project keeps content in Markdown files inside the `docs` directory. A simple way to organise translations is to create a folder for each language. For instance this repository provides `docs/en`, `docs/ru` and `docs/es`. The generated pages will then live at URLs such as `c4m1r.github.io/wiki/en/`.

Some static site generators, such as **Zola**, provide built-in support for multiple languages. Others may require a plugin or a custom folder layout.

## Using Zola

1. Add a `languages` section to your `config.toml` file and list the language codes you want to enable.
2. Create subdirectories like `content/en` and `content/ru` (or any other codes).
3. Place translated Markdown files in the matching directories.
4. Build the site with `zola build` and the generator will produce separate language versions.

Here is a minimal example from the official Zola docs:

```toml
[languages.en]
language_code = "en"

[languages.ru]
language_code = "ru"

[languages.es]
language_code = "es"
```

Create `content/en`, `content/ru` and `content/es` for translated pages. Add a language switcher link in your templates so readers can jump between them.

## Using mdBook

`mdBook` does not support multiple languages by default. A common approach is to maintain a separate book for each language and link between them. Plugins like `mdbook-i18n-helpers` can automate some tasks.

## Manual Approach

If your generator lacks built-in support, you can create folders such as `docs/en`, `docs/ru` and `docs/es`. Duplicate the structure for each language and link between them manually. Most generators will let you select which folder to build.

Whichever tool you choose, keeping file names consistent across languages makes it easier to maintain translations.
