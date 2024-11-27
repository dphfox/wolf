---
layout: page
---

Wolf is a compiled scripting language for expressing logic declaratively. It
supports multiple paradigms, modern safety features, and transpiles to
performant Luau code.

## Design

<nav>
	<ul>
	{% assign pages = site.pages | where_exp: 'page', 'page.dir == "/design/"' %}
	{% for page in pages %}
		<li><a href="{{page.url}}">{{page.title}}</a></li>
	{% endfor %}
	</ul>
</nav>