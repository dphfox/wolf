---
layout: page
---

Wolf is an expression-based scripting language aiming to be:

- **Easy.** Wolf lets you bring your own paradigms, and provides the tools to use
them ergonomically.
- **Small.** Wolf is built from first principles by composing together a few, well
chosen features that are easily identified and repurposed.
- **Smart.** Wolf code can be analysed 100% statically with smart inference, so
you don't have to spell things out for the computer.
- **Parallel.** Wolf code avoids prescribing a specific order or method of running
code, so it can be rearranged or broken up across threads for performance.



## Design

<nav>
	<ul>
	{% assign pages = site.pages | where_exp: 'page', 'page.dir == "/design/"' %}
	{% for page in pages %}
		<li><a href="{{page.url}}">{{page.title}}</a></li>
	{% endfor %}
	</ul>
</nav>