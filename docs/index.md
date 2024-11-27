---
layout: page
---

Wolf is a compiled scripting language for expressing logic declaratively. It
supports multiple paradigms, modern safety features, and transpiles to
performant Luau code.

## Design

{% assign pages = site.pages | where_exp: 'page', 'page.dir == "/design/"' %}
{% for page in pages %}
- [{{page.title}}]({{page.url}})
{% endfor %}