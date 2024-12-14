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

A function can be turned into an algorithm using the `algo` keyword.

```
my_algorithm := algo fn (2 + 2)
```

## Generic compatibility

Within an algorithm, all generic Wolf features are still available, and do not
behave differently.

```
my_algorithm := algo fn (
	message := "Two plus two is \(two + two)"
	two := 1 + 1
	message
)
```

## Execution passing

Execution can be passed to another algorithm, similarly to applying a function.

This is only allowed inside of `algo` blocks.

```
plus_two := algo x := num ( x + 2 )

four := algo ( plus_two 2 )
```

As with functions, the precedence of the operation allows for chaining:

```
plus_two := algo x := num ( x + 2 )
times_three := algo x := num ( x * 3 )

twelve := algo ( times_three plus_two 2 )
```

The apply operator `->` also works for algorithms.

```
plus_two := algo x := num ( x + 2 )
times_three := algo x := num ( x * 3 )

twelve := algo (2 -> plus_two -> times_three )
```

## Joining expressions

Expressions can be joined together by using the "do" operator; add the `do`
keyword before each expression to create a chain that's evaluated in explicit
order from start to finish.

This is only allowed inside of `algo` blocks.

```
eight := algo fn (do 2 + 2 do 4 + 4)
```

`do` can be chained with as many expressions as desired, and it can be broken
onto multiple lines naturally.

```
twelve := algo fn (
	do 2 + 2 
	do 4 + 4
	do 6 + 6
)
```