# Strings

The second basic data type in Wolf is `str` - a UTF-8 string of text with LF
line endings.

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

## Expressions in strings

String literals can contain a block expression by adding `=()`, with the block
contents inside. (TODO: revise to something less common?)

An equals `=` not directly before a `(` is interpreted literally.

```
"The answer is =(2 + 2)"

"I can still write = normally in strings"
```

## Escaped characters

Escape sequences can be inserted with `\`:

- `\n` for newlines
- `\t` for tabs
- `\"` for double quotes
- `\\` for backslashes
- `\=` for equals

```
"This is a \"perfectly normal\" string."
```

## Raw string literals

Raw string literals are defined with 3 or more double quotes; the number of
double quotes must be matched at the other end.

Raw string literals insert the contents exactly as it appears in the file.

```
"""None of "these" things will =(do) anything."""
"""""""I can even """include""" this raw string!""""""
```

## Count of graphemes

Use the count `#` operator to measure the number of UTF-8 graphemes in a string.

In UTF-8, graphemes correspond closely with individual rendered glyphs,
analogously to characters in ASCII.

```
$ wf -- #"Hello"
5

$ wf -- #""
0

$ wf -- #"😊"
1
```