---
layout: page
title: Strings
page_number: 2
---

In Wolf, strings are a UTF-8 string of text with LF line endings.

## String literals

String literals are written in double quotes `"`.

```
""
"Hello, world"
```

## Multiple line strings

String literals can span multiple lines without special syntax. If the starting
line has any indentation, any matching indentation characters are ignored on the
following lines, but extra indentation is still included.

```
"A much longer string.
Strings can span multiple lines."

	"The string might start somewhere along a line that's indented.
	If that's the case, then matching whitespace characters on following
	lines are ignored."

	"However,
		you can still include extra indentation,
	and it will work fine."
```

## Raw string literals

Raw string literals are defined with 3 or more double quotes; the number of
double quotes must be matched at the other end.

Raw string literals insert the contents exactly as it appears in the file,
ignoring indentation at the start of following lines.

```
"""None of "these" things will \n do anything."""
"""""""I can even """include""" this raw string!""""""
```