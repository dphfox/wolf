---
layout: page
title: Philosophy
---

These pages describe the perspective Wolf takes on certain programming paradigms.

## Concepts 
<nav>
	<ol class="multicol">
	{% assign pages = site.pages | where_exp: 'page', 'page.dir == "/01-philosophy/"' | where_exp: 'page', 'page.page_number' %}
	{% for page in pages %}
		<li><a href="{{page.url}}">{{page.title}}</a></li>
	{% endfor %}
	</ol>
</nav>