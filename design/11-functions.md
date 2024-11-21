# Functions

Function expressions define an expression to be applied on a piece of data.

Functions start with the `fn` keyword, followed by `of`, an input type
matcher, an arrow `->` and an output type matcher.

```
fn of bool -> num
```

The function body is then assigned after an `=`.

```
fn of bool -> num = 42
```

If the output type is directly inferable and doesn't need a name, it can be
omitted:

```
fn of bool = 42
```

The type matchers may be given names to make them into accessible locations.
This is especially useful when working with composite types.

```
fn of is_awesome: bool = if is_awesome { 42 } else { 0 }

fn of (
	from: num
	to: num
	ratio: num
) = (to - from) * ratio + to

fn of pair: (num; num) -> (
	sum: num
	difference: num
) =	{
	sum = pair[0] + pair[1]
	difference = pair[0] - pair[1]
}

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