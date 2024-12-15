---
layout: page
title: Functions
page_number: 11
---

In Wolf, functions unpack a piece of data, transforms it with expressions, and
returns a new piece of data.

Functions start with the `fn` keyword, followed by a block to act as the
function body.

```
fn ( 42 )
```

## Input matching

Functions can accept a paramater by writing a type matcher to unpack the
parameter. The unpacked data can then be used in the function body.

TODO: update type matching to allow for matching args without naming them and vice versa

```
fn {from := num, to := num, ratio := num} (
	(to - from) * ratio + to
)

fn pair: {num, num}  (
	{
		sum: pair.0 + pair.1
		diff: pair.0 - pair.1
	}
)
```

## Function names

The function itself may be assigned to an identifier so it may be referenced
elsewhere.

```
lerp := fn {from := num, to := num, ratio := num} (
	(to - from) * ratio + to
)
```

A function can be applied to a piece of data by writing the identifier to the left of the operand.
Function application takes precedence over other nearby operations, so a block `()` is
needed to evaluate expression operands.

```
factorial := fn x := num (
	if x == 1 ( x ) else ( factorial(x - 1) )
}

five_factorial := factorial 5
```

## First-class functions

Functions can be defined below the top level, e.g. inside other functions:

```
two_functions := fn (
	fn (
		42
	)
)

inner_function := two_functions ()
fourty_two := inner_function ()
```

Functions can be generated with various parameters this way, as they are allowed
to capture identifiers from the outer environment.

```
multiply_by := fn factor := num (
	fn input := num ( input * factor )
)

double := multiply_by 2
triple := multiply_by 3

twenty := double 10
sixty := triple twenty
```

## Function chains

Functions can naturally be chained due to their precedence.

```
foo := abs sin rad x
```

If desired for readability, order can be reversed with the apply operator `->`.

```
foo := x -> rad -> sin -> abs
```

## Running a function

To run a function from the command line, pass in the name of the
script, followed by an identifier. If the identifier holds a function, it will
be evaluated.

Any returned values will be printed.

```
$ wf script_name.wf do_something
```

Alternatively, you can shorten the command by just passing the script - it will
default to looking for an identifier called `main`.

```
$ wf script_name.wf
```