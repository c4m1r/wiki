# Hidden Pages

The wiki supports pages that do not appear in the main navigation. They are perfect for drafts, spoilers or other content you do not want in the table of contents.

There are two ways to hide a page:

1. Place the file inside the `docs/hidden/` directory or give the filename an underscore prefix (`_secret.md`).
2. Mark the page with a `hidden: true` flag in its front matter (useful with generators like **mdBook** or **Zola**).

```markdown
---
hidden: true
---
```

Hidden pages are still rendered and can be visited directly via links. To expose a secret, simply link to it from another page of the story.
