# Functions

Function expressions define an expression to be applied on a piece of data.

Functions start with the `fn` keyword, followed by an input type
matcher, an arrow `->` and an output type matcher.

The function body is then assigned after an `=`.

```
fn _ -> num = 42
```

If the output type matcher can be inferred with `_`, then it may be omitted.

```
fn _ = 42
```

The type matchers may be given names to make them into accessible locations.
This is especially useful when working with composite types.

```
fn (
	from: num
	to: num
	ratio: num
) = (to - from) * ratio + to

fn pair: (num; num) -> (sum: num; difference: num) = {
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