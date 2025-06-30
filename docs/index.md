# Welcome to the Wiki

This repository is the starting point for a small static website. Every page is written in Markdown and lives under the `docs` directory.

You can generate the site with any Markdown-aware static site generator, such as **mdBook** or **Zola**.

## Features

- Simple, readable folder layout.
- *Hidden page* support for drafts or secret content.
- Outline documentation about the **GLPI** service-management platform.
- Notes on interactive renderers and GUI libraries in **Rust**.

## Getting Started

1. Choose a static site generator (e.g. `mdbook` or `zola`).
2. Run the generator on the `docs` folder to build the HTML output.
3. Open the generated `index.html` in your browser.

Hidden pages are ignored by default, so remember to link to them explicitly if you want readers to find your secrets.

## Planned Sections

- **CRM GLPI** – deployment and everyday administration tips.
- **Rust Renderers** – exploring libraries like `wgpu` and `Bevy`.
- **Story** – a short narrative that demonstrates hidden pages in action.
- **Multi-language Support** – see [multi-language guidelines](multilanguage.md).

Have fun expanding the wiki!
