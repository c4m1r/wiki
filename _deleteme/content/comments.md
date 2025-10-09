---
title:
  ru: Системы комментариев
  en: Comment Systems
  es: Sistemas de comentarios
---

<!-- LANG: ru -->
Статические сайты не поставляются с базой данных, но вы все равно можете позволить читателям оставлять отзывы. Многие вики используют размещенные сервисы комментариев, которые полагаются на проблемы Git.

## GitHub и GitLab

Проекты, размещенные на GitHub или GitLab, могут встраивать комментарии через их трекеры проблем. Инструменты вроде **utterances** и **giscus** хранят обсуждения как цепочки проблем. Чтобы включить их:

1. Создайте репозиторий на GitHub или GitLab для хранения комментариев.
2. Добавьте фрагмент JavaScript, предоставленный сервисом, в ваши шаблоны страниц.
3. Каждая страница будет сопоставлена с одной проблемой, где посетители могут публиковать замечания.

Оба сервиса поддерживают OAuth логины, поэтому читателям нужна только учетная запись GitHub или GitLab.

## Gitea

Gitea имеет совместимый API, поэтому вы можете адаптировать тот же подход с использованием плагинов, таких как `gitea-comment` или пользовательских скриптов. Они позволяют создавать или извлекать проблемы для хранения сообщений.

## Самостоятельные решения

Если вы предпочитаете держать все под контролем, ознакомьтесь с проектами вроде **glosa**, **hubbub** или **talaria**. Они запускают легковесный сервер, который хранит комментарии в Git и предоставляет небольшой API для статических страниц.

Какое бы решение вы ни выбрали, размещение скрипта в единственном шаблоне сохраняет Markdown файлы чистыми, одновременно включая динамичное обсуждение.
<!-- END_LANG -->

<!-- LANG: en -->
Static sites don't ship with a database, but you can still let readers leave feedback. Many wikis use hosted comment services that rely on Git issues.

## GitHub and GitLab

Projects hosted on GitHub or GitLab can embed comments via their issue trackers. Tools like **utterances** and **giscus** store discussions as issue threads. To enable them:

1. Create a repository on GitHub or GitLab to hold the comments.
2. Add the JavaScript snippet provided by the service to your page templates.
3. Each page will map to a single issue where visitors can post remarks.

Both services support OAuth logins, so readers just need a GitHub or GitLab account.

## Gitea

Gitea has a compatible API, so you can adapt the same approach using plugins such as `gitea-comment` or custom scripts. They allow you to create or fetch issues to store the messages.

## Self-hosted Options

If you prefer to keep everything under your control, check out projects like **glosa**, **hubbub**, or **talaria**. They run a lightweight server that stores comments in Git and expose a small API for static pages.

Whatever solution you choose, placing the script in a single template keeps the Markdown files clean while enabling dynamic discussion.
<!-- END_LANG -->

<!-- LANG: es -->
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
<!-- END_LANG -->
