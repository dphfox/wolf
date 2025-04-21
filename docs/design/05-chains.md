---
layout: page
title: Chains
page_number: 5
---

Chains allow multiple expressions to be appended together.

## Simple chains

A chain is written as a series of expressions delimited by arrows `->`.
The final expression determines the value of the chain as a whole.

```
-- This entire chain evaluates to 8, the result of the final expression.
add [2, 5] -> cos 2 -> max [2, 4, 6, 8]
```

## Forward passing

The key feature that makes chains useful is forward passing. This is when the
result of the previous expression is used in the next expression.

In a chain, underscores `_` in an expression are populated with the value from
the previous expression.

```
-- The underscore is replaced with 4, so this whole chain evaluates to 14.
add [2, 2] -> add [_, 10]
```

## Automatic underscores

In some common cases, Wolf can infer where the first underscore goes.

- `-> func _` - Functions that accept data directly.
	- This includes single-value tuples like `func [_]` or `func [[_]]`.
- `-> func [_, a, b, c]` - Functions that accept a tuple, with one initial datum
from the chain.

For these cases, you can use the fat arrow `=>` instead. This tells Wolf to
insert the first underscore for you.

```
-- Manual:
[2, 5] -> max _ -> log2 _ -> ceil _ -> exp2 _

-- Automatic:
[2, 5] => max => log2 => ceil => exp2
```

You may mix thin arrows and fat arrows in the same pipeline.

```
[2, 5] => max -> divide [10, _]
```

Ordinary conversions between single-value tuples still apply; in particular, you
may omit them for single values.

```
-- These statements are equivalent.
2 -> add [_, 2] -> multiply [_, 10]
2 => add 2 => multiply 10
```