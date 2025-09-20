---
layout: page
title: Formatting
page_number: 0
---

The Wolf parser respects a few formatting conventions when parsing source code.

## File encoding

All Wolf files must be parseable as valid UTF-8. A file that is not valid UTF-8
is not a valid Wolf program.

Valid Wolf files may use either CRLF or LF line endings. However, during parsing,
all line endings are normalised to LF, ensuring program behaviour is consistent
regardless of line ending convention used.

## New lines

The only meaningful whitespace in Wolf are new lines. When the parser encounters
a new line, it will end any in-progress parsing if possible. An in-progress
parse will only continue if ending the parse would result in incomplete syntax.

Non-syntax errors are not considered when processing newlines.

## Comments

Unless in a delimited span, you can type two hypens `--` to skip processing the
rest of the line. All text until the next new line is treated as a comment.

```wolf
-- This is a comment.
-- Text after the `--` is not processed.
```

Long comments can be started with three or more hyphens `---`. Long comments do
not stop at new lines; they only stop when the same number of hyphens is found.

```wolf
---
This is a long comment.
No text is processed until the next sequence of three hyphens.
---

--------------------------------------------------------------------------------
You can use as many hyphens as you'd like, which means you can embed shorter
sequences of hyphens without ending the comment.

Like this: ---

Only these next few hyphens can end the comment.
--------------------------------------------------------------------------------
```