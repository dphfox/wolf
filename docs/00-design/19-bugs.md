---
layout: page
title: Bugs
page_number: 19
---

Wolf ensures that any compiled program can accept its full domain of values. 
Where possible, these checks are done at compile-time for best stability.

However, not all checks can be done ahead of time, for example due to hardware variances or unpredictable inputs.
For these cases, Wolf allows run-time correctness checks to be added.

## Basic use

The `bug` keyword represents an expression which can only be reached due to a bug in the program.
They indicate that computation cannot proceed because the program has not followed sound logic.

<!--wolf-->
```
let square_positive_number = fn [input : num]
	if input < 0 then bug
	if input != input then bug
	else input * input
```

## Comparison to errors

Wolf does not consider bugs to be errors; unlike errors which are expected issues that are accounted for in the logic of the program, bugs are explicitly meant to be impossible.
Reaching a bug represents an issue with the program itself that cannot be caught at compile-time.

There is explicitly no valid value for the resulting computation, or for any accessible program state.
As a result, bugs invalidate and terminate all other computations which depend on either of these.
In the common case, this can manifest as a process exit.

Note that the compiler is free to insert or rearrange bug expressions, so long as they are only inserted or moved within regions of the program where evaluating a bug would eventually occur under the current conditions.