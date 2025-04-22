---
layout: page
title: Functions
page_number: 8
---

Functions represent an expression done on an unknown datum.

## Basic use

A function can be defined anywhere in an expression.

Each function is formed of a few pieces:

- The `fn` keyword, to indicate a new function is being constructed.
- A capture that decomposes the input datum into names.
- A colon `:` to separate the names from the expression.
- The expression representing the body of the function.

```
let double: fn x: x * 2

let four: double 2
```

## Multi-line functions

More complex functions can use a block to span multiple lines without ending the
function declaration.

```
let lerp: fn [:ratio <num>, :from <num>, :to <num>] (
	let difference: to - from
	to + difference * ratio
)

let five: lerp [from: 0, to: 10, ratio: 0.5]
```