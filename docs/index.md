---
layout: page
---

Wolf is an experimental mid-level game development language, designed to allow game developers to achieve radically performant code with a simple mental model, a lean compiler, and quick iteration times. _It's a hobby project and nowhere near ready for use, with design and implementation constantly in flux._

- **Memory safe** - out parameters, first class pools & generational indices, and contextual allocators enforce a downwards flow of memory that's easy to reason about and lighting fast to allocate or drop.
- **Thread safe** - inspired by GPU pipelines, language-level thread groups help you cleanly spawn worker pools and enforce read/write restrictions to prevent data races and false sharing.
- **Procedural** - pipeline chains, function overloading, and implicit parameters give procedural code OOP-style ergonomics and autocomplete without paying the performance tax.

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