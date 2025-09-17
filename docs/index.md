---
layout: page
---

Wolf is an expression-based declarative scripting language, built to be:

- **Easy.** Bring your own paradigms, and use them ergonomically.
- **Small.** Built from first principles with only a few, well chosen features.
- **Smart.** Statically analysable, but doesn't require annotations everywhere.

The design and implementation of Wolf are early and highly experimental.

Insipred by
[Luau](https://luau.org/),
[Rust](https://rust-lang.org/),
[Hylo](https://www.hylo-lang.org/),
[Jai](https://jai.community/),
[Haskell](https://www.haskell.org/),
[Vale](https://vale.dev/),
[Zig](https://ziglang.org/),
and [Gleam](https://gleam.run/).

## Table of contents

<nav>
	<ul>
	{% assign pages = site.pages | where_exp: 'page', 'page.name == "index.md"' | where_exp: 'page', 'page.title' %}
	{% for page in pages %}
		<li><a href="{{page.url}}">{{page.title}}</a></li>
	{% endfor %}
	</ul>
</nav>