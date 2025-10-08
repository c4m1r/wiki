---
title:
  ru: Скрытые страницы
  en: Hidden Pages
  es: Páginas ocultas
---

<!-- LANG: ru -->
Вики поддерживает страницы, которые не отображаются в основной навигации. Они идеальны для черновиков, спойлеров или другого контента, который вы не хотите видеть в оглавлении.

Есть два способа скрыть страницу:

1. Поместите файл в папку `docs/hidden/` или дайте имени файла префикс подчеркивания (`_secret.md`).
2. Отметьте страницу флагом `hidden: true` в ее front matter (полезно с генераторами вроде **other static site generators** или **other static site generators**).

```markdown
---
hidden: true
---
```

Скрытые страницы все равно рендерятся и могут быть посещены напрямую через ссылки. Чтобы раскрыть секрет, просто свяжитесь с ним с другой страницы истории.
<!-- END_LANG -->

<!-- LANG: en -->
The wiki supports pages that do not appear in the main navigation. They are perfect for drafts, spoilers or other content you do not want in the table of contents.

There are two ways to hide a page:

1. Place the file inside the `docs/hidden/` directory or give the filename an underscore prefix (`_secret.md`).
2. Mark the page with a `hidden: true` flag in its front matter (useful with generators like **other static site generators** or **other static site generators**).

```markdown
---
hidden: true
---
```

Hidden pages are still rendered and can be visited directly via links. To expose a secret, simply link to it from another page of the story.
<!-- END_LANG -->

<!-- LANG: es -->
La wiki admite páginas que no aparecen en la navegación principal. Son perfectas para borradores, spoilers u otro contenido que no quieres en la tabla de contenido.

Hay dos formas de ocultar una página:

1. Coloca el archivo dentro del directorio `docs/hidden/` o dale al nombre de archivo un prefijo de guion bajo (`_secret.md`).
2. Marca la página con una bandera `hidden: true` en su front matter (útil con generadores como **other static site generators** o **other static site generators**).

```markdown
---
hidden: true
---
```

Las páginas ocultas aún se renderizan y se pueden visitar directamente a través de enlaces. Para exponer un secreto, simplemente enlázalo desde otra página de la historia.
<!-- END_LANG -->
