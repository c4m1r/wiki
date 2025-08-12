https://rust-lang.github.io/mdBook/continuous-integration.html

# wiki

This repository contains the Markdown sources for a simple static website. All documentation lives in the `docs` directory. The content is organised by language under folders like `docs/en`, `docs/ru` and `docs/es`. You can render the pages into HTML using your favorite static site generator, such as **mdBook** or **Zola**.

## Project Overview

The project showcases a lightweight wiki with a few extra touches:

- A short interactive story demonstrates how hidden pages can be used for Easter eggs.
- Guides cover the **GLPI** service-management platform in depth. Topics include
  introduction, installation, asset management, ticket workflows, plugins,
  everyday administration, and business tasks.
- Notes introduce tools for building high-performance renderers in **Rust**.
- A flexible folder layout lets you organize content in multiple languages.
- A landing page template (e.g. in Zola) can show recent activity in a GitHub-like timeline.

## Building the Site

1. Install a static site generator. We recommend `mdbook` (`cargo install mdbook`) or [Zola](https://www.getzola.org/).
2. Run the generator against the `docs` folder. For example:
   ```bash
   mdbook build docs
   ```
   or
   ```bash
   zola build docs
   ```
3. The generated site will appear in the output directory created by your tool. If you enable multiple languages, each folder under `docs` will become its own URL prefix (`en`, `ru`, `es`, and so on).

For multi-language sites, see [docs/multilanguage.md](docs/multilanguage.md) for tips.

No generator is bundled with this repository, so you are free to choose the one that best fits your workflow.

## Hidden Pages

Pages placed in `docs/hidden` or starting with an underscore (`_`) are treated as hidden. Static site generators typically ignore these files unless they are explicitly linked. You can also mark a page with `hidden: true` in its front matter. See [docs/hidden-pages.md](docs/hidden-pages.md) for details.


## Comments and Updates

See [docs/comments.md](docs/comments.md) for ways to enable Git-based comments. The landing page can also show recent activity in a GitHub-like feed to keep readers informed.
When using Zola you can implement this feed on the main index page with a custom template that lists recent commits or posts.

## Contributing

Feel free to expand the GLPI or Rust sections. Each Markdown file stands on its own, so you can add new topics under `docs/` and link them from the index.

Start exploring in [`docs/index.md`](docs/index.md), which links to the English, Russian and Spanish sections.

## Planned GLPI Articles

The GLPI guides will eventually cover more advanced subjects, such as:

- Monitoring ticket activity and generating reports
- Importing inventory data from other systems
- Managing multi-entity setups for large organizations
