---
layout: page
title: Algorithms
page_number: 12
---

Wolf doesn't use sequential execution by default to stay paradigm-agnostic.
However, sequential execution is useful for implementing iterative solvers or IO
procedures.

In Wolf, these are explicitly supported as *algorithms*. Algorithms are an
enhanced kind of function which has a defined execution order.

## Basic usage

A function can be turned into an algorithm by replacing `fn` with `algo`.

```
let my_algorithm := algo [] 2 + 2
```

## Generic compatibility

Within an algorithm, all generic Wolf features are still available, and do not
behave differently.

```
let my_algorithm := algo [] (
	let two := 1 + 1
	let message := "Two plus two is \(two + two)"
	message
)
```

## Passing execution

Execution can be passed to another algorithm, similarly to applying a function.
All calls to algorithms must be prefixed with the `await` keyword.

This is only allowed inside of `algo` blocks.

```
-- This is allowed.
let plus_two := algo [x / num] x + 2
let four := algo [] await plus_two 2

-- This is not allowed.
let four := fn [] plus_two 2
```

As with functions, the precedence of the operation allows for chaining:

```
let plus_two := algo [x / num] x + 2
let times_three := algo [x / num] x * 3
let twelve := await times_three await plus_two 2
```

The apply operators `->`/`=>` also work with algorithms.

```
plus_two := algo [x / num] x + 2
times_three := algo [x / num] x * 3
twelve := algo [] 2 => await plus_two => await times_three
```