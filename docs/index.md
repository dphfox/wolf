---
layout: page
---

Wolf is an expression-based scripting language designed by [Daniel P H Fox](https://phfox.uk/).

It is designed to be **easy**, **small**, **smart** and **parallel**.

- **Easy.** Wolf lets you bring your own paradigms, and provides the tools to use
them ergonomically.
- **Small.** Wolf is built from first principles by composing together a few, well
chosen features that are easily identified and repurposed.
- **Smart.** Wolf code can be analysed 100% statically with smart inference, so
you don't have to spell things out for the computer.
- **Parallel.** Wolf code avoids prescribing a specific order or method of running
code, so it can be rearranged or broken up across threads for performance.

The design and implementation of Wolf are early and highly experimental. Wolf
introduces new ideas about programming language design, but also draws
inspiration from many other languages, particularly
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