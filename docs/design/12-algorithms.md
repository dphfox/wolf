---
layout: page
title: Algorithms
page_number: 12
---

Wolf doesn't use sequential execution by default to stay paradigm-agnostic.
However, sequential execution is useful for implementing iterative solvers or IO
procedures.

In Wolf, these are explicitly supported as *algorithms*. Algorithms are an
enhanced kind of block which has a defined execution order.

## Basic usage

A block can be turned into an algorithm using the `algo` keyword.

TODO: could a literal ever require algo? e.g. `algo xyz` without the brackets

```
my_algorithm := algo (2 + 2)
```

Unlike normal blocks, algorithms are not evaluated automatically - execution has
to be explicitly passed to them by another algorithm.

## Generic compatibility

Within an algorithm, all generic Wolf features are still available, and do not
behave differently.

```
my_algorithm := algo (
	message := "Two plus two is \(two + two)"
	two := 1 + 1
	message
)
```

## Multiple expressions

Algorithms can contain multiple expressions separated by commas; they will be
evaluated from start to finish. The final expression determines the value the
algorithm will evaluate to after executing.

```
twelve := algo (2 + 2, 4 + 4, 6 + 6)
```

Algorithms can span multiple lines; if they do, then commas will be
automatically inserted if the next line looks like the start of a new
expression.

```
twelve := algo (
	2 + 2
	4 + 4
	6 + 6
)
```

TODO: argument/return values a la functions

## Execution passing

Execution can be passed to other algorithms by using the `->` operator.

This operator is available inside of `algo` blocks (including nested
blocks that are not explicitly `algo`), but not available in generic Wolf code.

```
four := algo (2 -> plus_two)
```

Algorithms can be chained together, passing the output from one as the input
to the next.

```
twelve := algo (2 -> plus_two -> times_three)
```