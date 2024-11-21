# Blocks

Blocks are a group of expressions that are evaluated in order. Blocks themselves
are expressions, so can wrap any expression anywhere in the program.

Blocks are defined with curly braces `{}`, containing at least one expression.

```
{ 2 + 2 }
```

Multiple expressions can be evaluated by separating them with commas `,`.

The last expression becomes the returned value.

```
{
	2 + 2, 4 + 4, 6 + 6
}
```

Blocks can span multiple lines; if they do, then commas will be automatically
inserted if the next line looks like the start of a new expression.

```
{
	2 + 2
	4 + 4
	6 + 6
}
```