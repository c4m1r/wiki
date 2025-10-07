# Sistemas de comentarios

Los sitios estáticos no vienen con una base de datos, pero aún puedes permitir que los lectores dejen comentarios. Muchas wikis utilizan servicios de comentarios alojados que dependen de problemas de Git.

## GitHub y GitLab

Los proyectos alojados en GitHub o GitLab pueden integrar comentarios a través de sus rastreadores de problemas. Herramientas como **utterances** y **giscus** almacenan discusiones como hilos de problemas. Para habilitarlos:

1. Crea un repositorio en GitHub o GitLab para almacenar los comentarios.
2. Agrega el fragmento de JavaScript proporcionado por el servicio a tus plantillas de página.
3. Cada página se mapeará a un solo problema donde los visitantes pueden publicar comentarios.

Ambos servicios admiten inicios de sesión OAuth, por lo que los lectores solo necesitan una cuenta de GitHub o GitLab.

## Gitea

Gitea tiene una API compatible, por lo que puedes adaptar el mismo enfoque usando complementos como `gitea-comment` o scripts personalizados. Te permiten crear o recuperar problemas para almacenar los mensajes.

## Opciones autoalojadas

Si prefieres mantener todo bajo tu control, echa un vistazo a proyectos como **glosa**, **hubbub** o **talaria**. Ejecutan un servidor ligero que almacena comentarios en Git y expone una pequeña API para páginas estáticas.

Cualquiera que sea la solución que elijas, colocar el script en una sola plantilla mantiene los archivos Markdown limpios mientras habilita la discusión dinámica.
