# Functions

In Wolf, functions unpack a piece of data, transforms it with expressions, and
returns a new piece of data.

Functions start with the `fn` keyword, followed by an input type
matcher, an arrow `->` and an output type annotation.

Then, a block is given to act as the function body.

```
fn of _ -> num ( 42 )
```

If the output type can be inferred, then it may be omitted.

```
fn of _ ( 42 )
```

The input type matcher may be given names to access parts of the incoming data.
This is especially useful when working with composite types.

```
fn of {from: num, to: num, ratio: num} (
	(to - from) * ratio + to
)

fn of pair: {num, num} (
	{
		sum: pair.0 + pair.1
		diff: pair.0 - pair.1
	}
)
```

The function itself may be given a name so it may be referenced elsewhere.

A function can be applied to a piece of data by writing the identifier to the left of the operand.
Function application takes precedence over other nearby operations, so a block `()` is
needed to evaluate expression operands.

```
factorial := fn of x: num (
	if x == 1 ( x ) else ( factorial(x - 1) )
}

five_factorial := factorial 5
```



## Running a function

To run a function from the command line, pass in the name of the
script, followed by a function identifier.

Any returned values will be printed.

```
$ wf script_name.wf do_something
```

Alternatively, you can shorten the command by just passing the script - it will
default to looking for a function called `main`.

```
$ wf script_name.wf
```