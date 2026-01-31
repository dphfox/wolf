---
layout: page
title: Functions
page_number: 11
---

Functions represent an expression done on an unknown datum.

## Basic use

A function can be defined anywhere in an expression.

Each function is formed of a few pieces:

- The `fn` keyword, to indicate a new function is being constructed.
- A tuple capture that decomposes the input datum into names.
- The expression representing the body of the function.

<!--wolf-->
```
let multiply_add = fn(x : num, y : num, z : num) x * y + z

let lerp = fn(
	.from a : num
	.to b   : num
	.ratio  : num
) (
	let difference = b - a
	a + difference * ratio
)

let four = multiply_add(5, 2, 3)
let five = lerp(.from 0, .to 10, .ratio 0.5)
```

## First-class functions

In Wolf, functions are first-class; that is, you can pass functions around like values.

In fact, all functions in Wolf are values; "freestanding" functions are simply function values assigned to names with `let`.

<!--wolf-->
```
let base_price = 50
let price_per_xp_level = 10

let final_price = price_info(
	.item "gem_sword"
	.dynamic_price fn(.xp_level) base_price + xp_level * price_per_xp_level
)
```

## Explicit types

Wolf only uses locally visible information to fill in missing type information.

As a result, functions stored in `let` assignments must always have explicit type information.
This is because `let` statements give no context about how the function will be used.

<!--wolf-->
```
-- Untyped captures like this are not allowed.
let multiply_add = fn(x, y, z) x * y + z
```

However, functions defined in other places may be able to draw on other context.
For example, a function "callback" passed into another function can draw on type information provided by the outer function.

<!--wolf-->
```
-- Untyped captures like this are allowed, because type information is available
-- for `.dynamic_price` here.
let final_price = price_info(
	.item "gem_sword"
	.dynamic_price fn(.xp_level) base_price + xp_level * price_per_xp_level
)
```