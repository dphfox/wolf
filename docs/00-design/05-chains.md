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
2 + 5 -> cos 2 -> max [2, 4, 6, 8]
```

## Forward passing

The key feature that makes chains useful is forward passing. This is when the
result of the previous expression is used in the next expression.

In a chain, underscores `_` in an expression are populated with the value from
the previous expression.

```
-- The underscore is replaced with 4, so this whole chain evaluates to 14.
2 + 2 -> _ + 10
```

## Automatic chaining

In some common cases, Wolf can infer where the first underscore goes.

- Functions that accept data directly (e.g. `-> func _`, `func [_]` or  `func [[_]]`).
- Functions that accept a tuple (e.g. `-> func [_, a, b, c]`)
- Operators [^1] (e.g. `_ + a`, `_...` or `+_`).

[^1]: Double-ended ranges (e.g. `a < _ <= b`) cannot be automatically chained.

For these cases, you can use the fat arrow `=>` instead. This tells Wolf to
insert the first underscore for you.

```
-- Manual:
[2, 5] -> max _ -> log2 _ -> ceil _ -> exp2 _

-- Automatic:
[2, 5] => max => log2 => ceil => exp2
```

You may mix thin arrows and fat arrows in the same chain.

```
[2, 5] => max -> 10 / _
```

Ordinary conversions between single-value tuples still apply; in particular, you
may omit them for single values.

```
-- These statements are equivalent.
2 -> add [_, 2] -> multiply [_, 10]
2 => add 2 => multiply 10
2 => + 2 => * 10
```