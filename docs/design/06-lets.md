---
layout: page
title: Lets
page_number: 6
---

Lets allow you to introduce names for the duration of a block.

## Basic use

Any number of lets can be defined in a block before the final expression.

Each let is formed of a few pieces:

- The `let` keyword, to indicate new names are being introduced.
- A *capture* that introduces names (this will be elaborated on later).
- A colon `:` to separate the names from the expression.
- The expression to be captured / named.

In the simplest case, a let introduces one name.

```
let four: 4
let negative_two: negate 2
```

Once a name is introduced anywhere in the block, it can be used anywhere else in
the block. Order does not matter.

```
-- Notice that `four` and `negative_two` are underneath `negative_eight`.
let negative_eight: four => multiply negative_two

let four: 4
let negative_two: negate 2
```

Conceptually, you can imagine replacing each name with that's name's expression.

```
-- The compiler sees this.
let negative_eight: (4) => multiply (negate 2)
```

## Restrictions

Expressions in lets must be resolvable without infinite cycles.

```
-- This is not allowed.
let two: four => subtract 2
let four: two => add 2
```

Additionally, a name cannot be introduced to the same block more than once, as
it is unclear which expression should be used.

```
-- This is not allowed.
let cool_number: 5
let cool_number: 42
```

## Nesting

Inner blocks can see names declared in outer blocks.

```
let five: 5
(
	-- This evaluates to 10.
	five * 2
)
```

## Shadowing

Inner blocks can redefine names from outer blocks; this is called shadowing.
Expressions in the inner block see the inner value, while expressions in the
outer block see the outer value.

```
let foo: 1
let fifty: (
	let foo: 5
	foo * 10 -- sees foo as `5`
)
let ten: foo * 10 -- sees foo as `1`
```

## Empty lets

An "empty let" is a let without a colon or expression. They introduce names
without introducing any expressions or values.

```
-- This is an example of an empty let.
let foo
```

It's a compile error to evaluate an empty let.

```
-- This is not allowed.
let foo
foo * 10
```

Empty lets can be used to prevent inner blocks from using a name declared in an
outer block.

```
let foo: 42
(
	-- Overshadows the outer `foo`, but doesn't provide a value.
	let foo
	
	-- As a result, it is a compile error to try and use `foo` here.
	foo * 10
)
```