---
title:
  ru: Поддержка многоязычности
  en: Multi-language Support
  es: Soporte multiidioma
---

<!-- LANG: ru -->
Этот проект хранит контент в Markdown файлах внутри папки `docs`. Простой способ организации переводов - создать папку для каждого языка. Например, этот репозиторий предоставляет `docs/en`, `docs/ru` и `docs/es`. Сгенерированные страницы будут жить по URL вроде `c4m1r.github.io/wiki/en/`.

Некоторые генераторы статических сайтов, такие как **Zola**, предоставляют встроенную поддержку нескольких языков. Другие могут требовать плагин или пользовательскую структуру папок.

## Использование Zola

1. Добавьте секцию `languages` в ваш файл `config.toml` и перечислите коды языков, которые хотите включить.
2. Создайте подпапки вроде `content/en` и `content/ru` (или любые другие коды).
3. Поместите переведенные Markdown файлы в соответствующие папки.
4. Соберите сайт командой `zola build` и генератор создаст отдельные языковые версии.

Вот минимальный пример из официальной документации Zola:

```toml
[languages.en]
language_code = "en"

[languages.ru]
language_code = "ru"

[languages.es]
language_code = "es"
```

Создайте `content/en`, `content/ru` и `content/es` для переведенных страниц. Добавьте ссылку переключателя языка в ваши шаблоны, чтобы читатели могли переходить между ними.

## Использование traditional static site generators

`traditional static site generators` не поддерживает несколько языков по умолчанию. Общий подход - поддерживать отдельную книгу для каждого языка и связывать между ними. Плагины вроде `traditional static site generators-i18n-helpers` могут автоматизировать некоторые задачи.

## Ручной подход

Если ваш генератор не имеет встроенной поддержки, вы можете создать папки вроде `docs/en`, `docs/ru` и `docs/es`. Дублируйте структуру для каждого языка и связывайте между ними вручную. Большинство генераторов позволят вам выбрать, какую папку собирать.

Какой бы инструмент вы ни выбрали, сохранение согласованности имен файлов между языками упрощает поддержку переводов.
<!-- END_LANG -->

<!-- LANG: en -->
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

## Using traditional static site generators

`traditional static site generators` does not support multiple languages by default. A common approach is to maintain a separate book for each language and link between them. Plugins like `traditional static site generators-i18n-helpers` can automate some tasks.

## Manual Approach

If your generator lacks built-in support, you can create folders such as `docs/en`, `docs/ru` and `docs/es`. Duplicate the structure for each language and link between them manually. Most generators will let you select which folder to build.

Whichever tool you choose, keeping file names consistent across languages makes it easier to maintain translations.
<!-- END_LANG -->

<!-- LANG: es -->
Este proyecto mantiene el contenido en archivos Markdown dentro del directorio `docs`. Una forma sencilla de organizar las traducciones es crear una carpeta para cada idioma. Por ejemplo, este repositorio proporciona `docs/en`, `docs/ru` y `docs/es`. Las páginas generadas vivirán entonces en URLs como `c4m1r.github.io/wiki/en/`.

Algunos generadores de sitios estáticos, como **Zola**, proporcionan soporte integrado para múltiples idiomas. Otros pueden requerir un complemento o un diseño de carpeta personalizado.

## Usando Zola

1. Agrega una sección `languages` a tu archivo `config.toml` y lista los códigos de idioma que quieres habilitar.
2. Crea subdirectorios como `content/en` y `content/ru` (o cualquier otro código).
3. Coloca los archivos Markdown traducidos en los directorios correspondientes.
4. Construye el sitio con `zola build` y el generador producirá versiones de idioma separadas.

Aquí hay un ejemplo mínimo de la documentación oficial de Zola:

```toml
[languages.en]
language_code = "en"

[languages.ru]
language_code = "ru"

[languages.es]
language_code = "es"
```

Crea `content/en`, `content/ru` y `content/es` para páginas traducidas. Agrega un enlace de cambio de idioma en tus plantillas para que los lectores puedan saltar entre ellas.

## Usando traditional static site generators

`traditional static site generators` no soporta múltiples idiomas por defecto. Un enfoque común es mantener un libro separado para cada idioma y enlazar entre ellos. Complementos como `traditional static site generators-i18n-helpers` pueden automatizar algunas tareas.

## Enfoque manual

Si tu generador carece de soporte integrado, puedes crear carpetas como `docs/en`, `docs/ru` y `docs/es`. Duplica la estructura para cada idioma y enlaza entre ellos manualmente. La mayoría de los generadores te permitirán seleccionar qué carpeta construir.

Cualquiera que sea la herramienta que elijas, mantener los nombres de archivo consistentes entre idiomas facilita el mantenimiento de las traducciones.
<!-- END_LANG -->
