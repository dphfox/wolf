# Functions

Functions contain a sequence of instructions to be run one after the other. All
sequential code in Wolf lives inside of some kind of function.

A simple function starts with the `fn` keyword, followed by an identifier, an
`=`, and an expression:

```
fn do_something = 2 + 2
```

Function identifiers are usually `snake_case`.

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