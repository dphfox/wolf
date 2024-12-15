---
layout: page
title: Streams
page_number: 13
---

Streams form the basis of many kinds of bulk data processing and iteration in
Wolf.

## Construction

The stream operator `..` can be used to construct streams from various things.
It is available in generic Wolf code.

It can be prefixed to numbered tables to create a stream of the values.

TODO: the most generic form streams pairs from tables, could a type matcher be
used there to make it ergonomic?

```
data := {1, 1, 2, 3, 5, 8}
iter := ..data

iter2 := ..({1, 2, 4, 8})
```

It can also be used infix to iterate over a range of values defined by two
endpoints - the start value is inclusive, but the end value is exclusive. Again,
this can be implemented for different types:

```
min := 0
max := 10
count_up_inclusive := min..(max + 1)

infinite_count_up := min..inf
```

## For expressions

For expressions can be used to apply an expression to every value in a stream.

The most basic form is comprised of the `for` keyword, an identifier, the `in`
keyword, an expression evaluating to a stream, and a block containing the
expression to be applied.

```
multiples_of_5 := for x in 0..inf ( x * 5 )
```

A condition can be added to filter out values from the stream after the block.

The condition is specified using `if` followed by a predicate expression. The
condition can depend on identifiers from the block.

```
even_squares := for x in 0..inf (
	square := x ^ 2
	square
) if square % 2 == 0
```

If filtering is all that's needed, the block can be omitted.

```
even_numbers := for x in 0..inf if x % 2 == 0
```

