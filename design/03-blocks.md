# Block expressions

Blocks evaluate expressions inside of them to produce a value.

## Basic usage

Blocks are declared with parentheses `()`.

```
(2 + 2)
```

Blocks can be included in other expressions. The contents of the block are
evaluated independently of the other expression.

```
2 * (13 - 5)
```

## Multiple expressions

Blocks can contain more than one expression, separated by commas `,`.

The value of the last expression will be used as the value for the whole block.

```
(2 + 2, 4 + 4, 6 + 6)
```

Blocks can span multiple lines; if they do, then commas will be automatically
inserted if the next line looks like the start of a new expression.

```
(
	2 + 2
	4 + 4
	6 + 6
)
```