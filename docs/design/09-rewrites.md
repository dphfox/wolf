---
layout: page
title: Rewrites
page_number: 9
---

Rewriting allows you to overshadow an existing declaration, as well as
everything that depends on that declaration, for the duration of a block.

## Basic use

Rewrites look identical to a `let` declaration, except they use the `rewrite`
keyword. They must point at an existing declaration.

The new value does not have to be the same kind of data.

```
let some_thing: 25
(
	rewrite some_thing: "new value"
)
```

Similarly to shadowing, anything inside of the current block will see the 
rewritten value whenever they try to access the original declaration.

```
let cool_number: 5

-- `result1` evaluates to -42.
let result1: (
	rewrite cool_number: 42
	negate cool_number
)

-- `result2` evaluates to -5
let result2: negate cool_number
```

As with ordinary `let` declarations, you may not define the same name more than
once in the same block.

```
-- This is not allowed - it's ambiguous what the value of `some_thing` is
-- inside of this block.
let some_thing: 25
rewrite some_thing: "new value"
```

## Empty lets

If there's no sensible default value for the original declaration, you can use
an empty `let` declaration to introduce names for rewriting later.

This is useful for declarations whose values are only known during certain parts
of the program.

```
let foo
let foo_plus_two: foo + 2

-- The `rewrite` makes it valid to use `foo` and its dependencies.
let ten_plus_two = (
	rewrite foo: 10
	foo_plus_two
)
```

## Dependencies

Unlike shadowing, `rewrite` rewrites everything that depends on the original
declaration, even dependencies declared outside of the block.

This property allows parts of the program to use different values for the same
declaration, for example to share light/dark theme information to UI components,
or to inject mock functions for testing.

```
let total: fn [a, b, c] (a => add b => add c)

-- Rewrites `total` to use the new `add`.
let rewriting: (
	rewrite add: fn [a, b] (100)
	total [2, 4, 6] -- evaluates to 100
)

-- Does not rewrite the `add` in `total`.
let shadowing: (
	let add: fn [a, b] (100)
	total [2, 4, 6] -- evaluates to 12
)
```

This transcription is done at compile time. You might imagine the declarations
being "copied into" the block.

```
-- The compiler sees something similar to this.
let rewriting: (
	let add: fn [a, b] (100)
	let total: fn [a, b, c] (a => add b => add c)
	total [2, 4, 6] -- evaluates to 100
)
```
