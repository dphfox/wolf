---
layout: page
title: Looping
page_number: 12
---

Looping allows part of an expression chain to feed back into itself.

## Basic use

After an arrow, the `loop` keyword can be added to signal that the following
expression can be fed back into itself.

The expression must either:
- Return a value wrapped in `feedback [_]` (to feed the value back into the expression).
- Return a value wrapped in `break [_]` (which will exit the loop and continue along the chain). 

It is invalid to return a value which is not wrapped.

```wolf
largest_multiple_of_two = fn [.no_larger_than : num] (
	2 -> loop (
		let current = _
		let doubled = current * 2
		if doubled > no_larger_than
			then break [current]
			else feedback [doubled]
	)
)

sixty_four = largest_multiple_of_two [.no_larger_than 80]
```

## Equivalence

Loops are equivalent to recursion - they are a more ergonomic way of writing
a recursive function:

```wolf
-- This is equivalent to the loop above.
largest_multiple_of_two = fn [.no_larger_than : num] (
	feedback = fn [current] (
		let doubled = current * 2
		if doubled > no_larger_than
			then current
			else feedback [doubled]
	)
	feedback [2]
)
```

Loops are *also* equivalent to more traditional `for` or `while` constructs - 
the mutable state is the value that's fed back into the loop.

```wolf
-- This is also equivalent to the loop above (Luau syntax).

local function largest_multiple_of_two(no_larger_than: number)
	local current = 2
	while true do
		local doubled = current * 2
		if doubled > no_larger_than
			then break
			else current = doubled; continue
		end
	end
	return current
end
```

Because of this, loops are a useful paradigm-agnostic replacement for both
recursion _and_ `for`/`while` constructs.