# Páginas ocultas

La wiki admite páginas que no aparecen en la navegación principal. Son perfectas para borradores, spoilers u otro contenido que no quieres en la tabla de contenido.

Hay dos formas de ocultar una página:

1. Coloca el archivo dentro del directorio `docs/hidden/` o dale al nombre de archivo un prefijo de guion bajo (`_secret.md`).
2. Marca la página con una bandera `hidden: true` en su front matter (útil con generadores como **mdBook** o **Zola**).

```markdown
---
hidden: true
---
```

Las páginas ocultas aún se renderizan y se pueden visitar directamente a través de enlaces. Para exponer un secreto, simplemente enlázalo desde otra página de la historia.
