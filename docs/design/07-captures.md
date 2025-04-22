---
layout: page
title: Captures
page_number: 7
---

Captures describe how to name parts of a datum.

## Names

As seen before, the simplest capture is a name on its own.

In this case, the name refers to the datum itself.

```
-- `name` refers to `4`.
let name: 4
```

## Shapes

By using angle brackets `< >` after the name, the *shape* of the data can be
restricted. This limits what kinds of data are allowed to be captured by the
name.

Wolf provides a few built-in shapes, for example `num` for numbers and `str` for
strings.

```
let cool_number <num>: 4

let haiku <str>:
	"This must be a string
	Otherwise it won't compile
	So here's a haiku"
```

## Tuples

Instead of a name and/or shape, tuple syntax can be used to deconstruct a
tuple datum.

Names and shapes can be specified in place of data.

```
-- Simple tuple capture.
let [first, second, third]: [1, 2, 3]

-- Shapes can be included.
let [first_name <str>, <num>]: ["Adam", 27]

-- Named tuple capture.
let [first_name: first_name <str>, age: <num>]: [first_name: "Adam", age: 27]

-- Shorthand for `first_name: first_name`.
let [:first_name <str>, age: <num>]: [first_name: "Adam", age: 27]
```

The rest of a tuple's data can be captured at the end using `...`. (The shape is
optional.)

```
let [:first_name <str>, ... rest <[age: 27]>]: [first_name: "Adam", age: 27]
```