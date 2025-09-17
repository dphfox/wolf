---
layout: page
title: Chains
page_number: 7
---

Chains allow multiple expressions to be appended together.

## Simple chains

A chain is written as a series of expressions delimited by arrows `->`.
The final expression determines the value of the chain as a whole.

```
-- This entire chain evaluates to 8, the result of the final expression.
2 + 5 -> cos [2] -> max [2, 4, 6, 8]
```

## Forward passing

The key feature that makes chains useful is forward passing. This is when the
result of the previous expression is used in the next expression.

The underscore `_` name is reserved by Wolf. In a chain, it refers to the result 
of the previous expression.

```
-- The underscore is replaced with 4, so this whole chain evaluates to 14.
2 + 2 -> _ + 10
```

## Automatic chaining

In some common cases, Wolf can infer where the first underscore goes.

- Functions that accept data directly (e.g. `-> my_func [_]`  or `-> my_func [[_]]`).
- Functions that accept a tuple (e.g. `-> my_func [_, a, b, c]`)
- Operators [^1] (e.g. `_ + a`, `_...` or `+_`).

[^1]: Double-ended ranges (e.g. `a < _ <= b`) cannot be automatically chained.

For these cases, you can use the fat arrow `=>` instead. This tells Wolf to
insert the first underscore for you.

```
-- Manual:
[2, 5] -> max [_] -> log2 [_] -> ceil [_] -> exp2 [_]
2 -> _ + 5 -> _ * 3

-- Automatic:
[2, 5] => max => log2 => ceil => exp2
2 => + 5 => * 3
```

You may mix thin arrows and fat arrows in the same chain.

```
[2, 5] => max -> 10 / _
```