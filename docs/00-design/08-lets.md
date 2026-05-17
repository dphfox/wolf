---
layout: page
title: Lets
page_number: 8
---

Lets allow you to introduce names inside the scope of a tuple.

## Basic use

Any number of lets can be defined in a tuple.

Each let is formed of a few pieces:

- The `let` keyword, to indicate new names are being introduced.
- A *capture* that introduces names (this will be elaborated on later).
- An equals `=` to separate the names from the expression.
- The expression to be captured / named.

In the simplest case, a let introduces one name.

<!--wolf-->
```
let four = 4
let negative_two = negate(2)
```

Declarations happen top-down; earlier declarations may only be used by later declarations in a block.

<!--wolf-->
```
-- This is allowed.
let four = 4
let negative_two = negate(2)
let negative_eight = four => multiply(negative_two)

-- This is not allowed.
let negative_eight = four => multiply(negative_two)
let four = 4
let negative_two = negate(2)
```

Conceptually, you can imagine replacing each name with that's name's expression.

<!--wolf-->
```
-- The compiler sees this.
let negative_eight = 4 => multiply(negate(2))
```

## Nesting

Inner tuples can see names declared in outer tuples.

<!--wolf-->
```
let five = 5
(
	-- This evaluates to 10.
	five * 2
)
```

## Shadowing

Inner tuples can redefine names from outer tuples; this is called shadowing.
Expressions in the inner tuple see the inner value, while expressions in the outer tuple see the outer value.

<!--wolf-->
```
let foo = 1
let fifty = (
	let foo = 5
	foo * 10 -- sees foo as `5`
)
let ten = foo * 10 -- sees foo as `1`
```

Shadowing may also be done in the same block, in which case earlier declarations are shadowed by later definitions.

<!--wolf-->
```
let foo = 1
let ten = foo * 10 -- sees foo as `1`
let foo = 5
let fifty = foo * 10 -- sees foo as `5`
```