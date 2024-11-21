# Functions

In Wolf, functions unpack a piece of data, transforms it with expressions, and
returns a new piece of data.

Functions start with the `fn` keyword, followed by an input type
matcher, an arrow `->` and an output type matcher.

Then, a block is given to act as the function body.

```
fn _ -> num
{
	42
}
```

The type matchers may be given names to make them into accessible locations.
This is especially useful when working with composite types.

```
fn (from: num, to: num, ratio: num) -> num
{
	(to - from) * ratio + to
}

fn pair: (num, num) -> (sum: num, diff: num)
{
	sum = pair[0] + pair[1]
	diff = pair[0] - pair[1]
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