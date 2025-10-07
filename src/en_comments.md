# Comment Systems

Static sites don\'t ship with a database, but you can still let readers leave feedback. Many wikis use hosted comment services that rely on Git issues.

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
