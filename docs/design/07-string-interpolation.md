---
layout: page
title: String interpolation
page_number: 7
---

Custom substrings can be added into a string literal at any point with `\`:

- `\n` adds a new line
- `\t` adds a tab
- `\"` adds a double quote
- `\\` adds a backslash
- `\()` adds the result of a block, formatted for display
- `\?()` adds the result of a block, formatted for debugging

```
"This is a \"perfectly normal\" string."

"The answer is \(2 + 2)"
```

It isn't valid to use `\` without one of those specific sequences.

```
-- Not allowed:
"do \ re \ mi"

-- Allowed:
"do \\ re \\ mi"
```

TODO: should these be string templates a la Python instead?