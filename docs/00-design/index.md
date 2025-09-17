---
layout: page
title: Design
---

These pages give an overview of Wolf syntax and language features.

## Features 
<nav>
	<ol class="multicol">
	{% assign pages = site.pages | where_exp: 'page', 'page.dir == "/00-design/"' | where_exp: 'page', 'page.page_number' %}
	{% for page in pages %}
		<li><a href="{{page.url}}">{{page.title}}</a></li>
	{% endfor %}
	</ol>
</nav>