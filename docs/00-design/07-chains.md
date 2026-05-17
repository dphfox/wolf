---
layout: page
title: Chains
page_number: 7
---

Chains allow multiple expressions to be appended together.

## Basic use

Wolf provides the thin arrow `->` operator, to allow previous expressions to be piped into later expressions.

The thin arrow stores the result of the previous expression in `@`.

<!--wolf-->
```
-- `@` is replaced with 4, so this whole chain evaluates to 14.
2 + 2 -> @ + 10
```

## Fat arrow

If `@` is being passed as `.0` to a function, the fat arrow `=>` can be used to omit it.

<!--wolf-->
```
-- Manual:
(2, 5) -> max(@) -> log2(@) -> ceil(@) -> exp2(@)

-- Automatic:
(2, 5) => max() => log2() => ceil() => exp2()
```

You may mix thin arrows and fat arrows in the same chain.

<!--wolf-->
```
(2, 5) => max -> 10 / @
```

## Implicit chaining

Tuples with multiple expressions will implicitly chain those expressions with the thin arrow operator.

This form still permits use of `@` to reference the prior expression.

<!--wolf-->
```
-- These two expressions are identical
( 2 + 2 -> @ + 10 )

(
    2 + 2
    @ + 10
)
```