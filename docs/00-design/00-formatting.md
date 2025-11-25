---
layout: page
title: Formatting
page_number: 0
---

The Wolf parser respects a few formatting conventions when parsing source code.

## File encoding

Files are expected to be ASCII or UTF-8. Other encodings are not supported.

Valid Wolf files may use either CRLF or LF line endings. CR line endings are not supported.

The choice of encoding has no effect on program execution; it is not detectable.

## New lines

Syntax doesn't run onto new lines unless it is incomplete.

<!--wolf-->
```
-- This is interpreted as running onto the next line.
let foo = 2 -
5

-- This is interpreted as two statements on different lines.
let foo = 2
-5
```

## Comments

Unless in a delimited span, you can type two hypens `--` to skip processing the rest of the line.
All text until the next new line is treated as a comment.

<!--wolf-->
```
-- This is a comment.
-- Text after the `--` is not processed.
```

Long comments can be started with three or more hyphens `---`. 
Long comments do not stop at new lines; they only stop when the same number of hyphens is found.

<!--wolf-->
```
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