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

## Principles

Wolf is designed to be **easy**, **small**, **smart** and **parallel**. To
achieve this, a few principles have been derived that guide the design of
language features to reinforce these four tenets.

### Topological execution order

Wolf aims to bake parallelism deeply into the language. To emphasise this in the
language design, code is not run "top to bottom" or "left to right", as most
contemporary languages do.

Instead, execution is ordered *topologically*. That is, code only runs when
everything it needs is available. Code is free to run side by side with other
code; there isn't a singleton "instruction pointer" to follow.

That doesn't mean Wolf should look alien. In service of Wolf's easiness, the
syntax should feel like a natural extension of existing languages. All it needs
to do is make the topological relationships clear.

### As static as possible

To make Wolf smart, Wolf needs to know as much about the program as possible.
With more information, errors can be caught earlier in the development process,
IDEs can provide targeted suggestions, and code can be further optimised.

That's why Wolf programs are designed to be evaluated primarily at compile time.
Only truly unknowable parts of the program (such as user input or network IO)
are evaluated on the fly.

### Repeated patterns

To keep Wolf small and easy to understand, the core features of the language are
designed to map 1 to 1 with broad concepts, and reused in as many places as
possible.

Beyond raw function, the appearance of the language also reflects this reuse.
The syntax of features is designed to be context independent, so that the same
syntax can be used everywhere a feature appears in the language, with minimal
special cases.