---
layout: page
title: Chains
page_number: 7
---

Chains allow multiple expressions to be appended together.

## Simple chains

A chain is written as a series of expressions delimited by arrows `->`.
The final expression determines the value of the chain as a whole.

<!--wolf-->
```
-- This entire chain evaluates to 8, the result of the final expression.
2 + 5 -> cos [2] -> max [2, 4, 6, 8]
```

## Forward passing

Wolf stores the result of the previous expression in `that`.
This allows previous expressions to be used in the next expression.

<!--wolf-->
```
-- `that` is replaced with 4, so this whole chain evaluates to 14.
2 + 2 -> that + 10
```

## Automatic chaining

If `that` is being passed as `.0` to a function, the fat arrow `=>` can be used to omit it.

<!--wolf-->
```
-- Manual:
[2, 5] -> max [that] -> log2 [that] -> ceil [that] -> exp2 [that]

-- Automatic:
[2, 5] => max [] => log2 [] => ceil [] => exp2 []
```

This works with 

You may mix thin arrows and fat arrows in the same chain.

<!--wolf-->
```
[2, 5] => max -> 10 / that
```