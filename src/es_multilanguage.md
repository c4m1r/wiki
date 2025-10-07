# Soporte multiidioma

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

## Usando mdBook

`mdBook` no soporta múltiples idiomas por defecto. Un enfoque común es mantener un libro separado para cada idioma y enlazar entre ellos. Complementos como `mdbook-i18n-helpers` pueden automatizar algunas tareas.

## Enfoque manual

Si tu generador carece de soporte integrado, puedes crear carpetas como `docs/en`, `docs/ru` y `docs/es`. Duplica la estructura para cada idioma y enlaza entre ellos manualmente. La mayoría de los generadores te permitirán seleccionar qué carpeta construir.

Cualquiera que sea la herramienta que elijas, mantener los nombres de archivo consistentes entre idiomas facilita el mantenimiento de las traducciones.
