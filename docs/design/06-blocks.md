---
layout: page
title: Blocks
page_number: 6
---

Blocks evaluate expressions inside of them to produce a value. 

They can be used to embed expressions safely inside of other expressions.

## Basic usage

Blocks are declared with parentheses `()`. The inner expression becomes the
outer value.

```
-- This block evaluates to 2.
(2)

-- This block evaluates to -2.
(negate 2)
```

Blocks can be included in other expressions. The contents of the block are
evaluated independently of the other expression.

```
cos (negate 2)
```
