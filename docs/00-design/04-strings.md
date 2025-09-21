---
layout: page
title: Strings
page_number: 4
---

In Wolf, strings are a UTF-8 string of text with LF line endings.

## String literals

String literals are written in double quotes `"`.

<!--wolf-->```
""
"Hello, world"
```

## Multiple line strings

String literals can contain line breaks. The line feed character is not included
in the string.

If the starting line has any indentation, any matching indentation characters
are ignored on the following lines. Extra indentation characters remain included.

<!--wolf-->```
"A much longer string.
Strings can span multiple lines."

	"The string might start somewhere along a line that's indented.
	If that's the case, then matching whitespace characters on following
	lines are ignored."

	"However,
		you can still include extra indentation,
	and it will work fine."
```

## Insert sequences

Backslashes `\` in strings indicate the start of an insert sequence, which can
be used to change how the string is interpreted:

- `\n` inserts a line feed character
- `\t` inserts a tab character
- `\\` inserts a literal backslash
- `\"` inserts a literal quote

<!--wolf-->```
"By explicitly inserting a line break \n
this multiline string is able to keep the line feed character."

"Backslashes let you use \"quotes\" around part of the string!"
```

## Raw string literals

Raw string literals are defined with 3 or more double quotes; the number of
double quotes must be matched at the other end.

Raw string literals ignore insert sequences and preserve line feed characters.

<!--wolf-->```
"""None of "these" things will \n do anything."""
"""""""I can even """include""" this raw string!""""""

		"""The original line's indentation won't be included,
			but the extra indent here *will* be included,
		as well as all of the line breaks used here."""
```