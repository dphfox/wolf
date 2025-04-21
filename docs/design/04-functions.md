---
layout: page
title: Functions
page_number: 4
---

Functions are used to process and transform data in Wolf.

## Basic use

To use a function, prefix your data with the name of the function. A space may
be necessary for some kinds of data, like numbers.

```
-- Negates 2.
negate 2
```

## Composition

You can run multiple functions on one piece of data by adding more names. This
is known as function composition.

```
-- Negates 2, then takes the cosine of -2.
cos negate 2
```

Function composition has very high precedence. That means the function always
applies to whatever is directly right of the name.

```
-- This is how the statement is interpreted by the compiler.
cos (negate 2)
```

## Multiple data

Unlike other languages, Wolf functions don't accept more than one piece of data
by default. When multiple "arguments" are needed, the data is physically grouped
together, often with a tuple.

For example, the built-in `add` function takes a tuple, which emulates the
"traditional" list of arguments.

```
-- Adds 9 and 10.
add [9, 10]
```

However, there is nothing special about the above function; it still accepts a
single datum.