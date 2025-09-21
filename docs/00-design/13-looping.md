---
layout: page
title: Looping
page_number: 13
---

Looping allows an expression to run repeatedly and capture the output from the
last iteration.

## Basic use

A loop is formed of a few parts:

- The `loop` keyword.
- A capture to use on every iteration.
- The assignment operator `=`.
- The initial value to be captured.

<!--wolf-->
```
-- Start at 1, and add 1 each time.
let count_up_forever = loop x = 1 (x + 1)
```

## Throwing

By default, a loop will never terminate. This means you will need to set an exit
condition in order for the loop to be useful.

So, loops can be thrown values, just like `catch` blocks can. When this happens,
the loop terminates, and evaluates to the thrown value.

<!--wolf-->
```
-- Find the largest multiple of two that's less than `limit`.
let largest_mult = fn [.limit : num] loop n = 2 (
	if 2 * n > limit then throw n
	else n * 2
)

let sixteen = largest_mult [.limit 25]
```

## Multiple data

Using tuple captures, a loop can keep track of multiple named data with every
iteration.

<!--wolf-->
```
let sum_of_consecutive = loop [index, total] = [10, 0] (
	if index == 0 then throw total
	else [index - 1, total + index]
)
```