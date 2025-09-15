---
layout: page
title: Context
page_number: 14
---

Functions may depend on extra helpers or extra knowledge that aren't related to
the main datum they operate on.

Context allows a function to access these extra features transparently.

## Basic use

Names of contexts can be declared in braces `{}` after the `fn` keyword.

They must refer to items accessible in the current namespace.

```

let add_my_secret_number := fn {secret_number} [x / num] x + secret_number
```