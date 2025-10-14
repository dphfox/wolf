---
layout: page
title: Conditions
page_number: 12
---

Code often needs to do one thing or another, dependent on a condition being true or false.

Wolf provides conditional expressions for evaluating one expression or another based on the outcome of a condition expression.

## Basic use

A conditional expression is made of a few parts:

- The `if` keyword.
- The condition: an expression that evaluates to a bool.
- The `then` keyword.
- The "true branch": an expression to be used when the condition is `true`.
- The `else` keyword.
- The "false branch": an expression to be used when the condition is `false`.

All parts of the construct are required.

<!--wolf-->
```
let get_account_type = fn [age : num] if age < 18 then "Child" else "Adult"
```

The true and false branches must have a compatible type.

<!--wolf-->
```
-- This is not allowed.
let get_account_type = fn [age : num] if age < 18 then "Child" else 2
```

## Multiple conditions

Conditional expressions can be composed together in order to test more than one condition at a time.

<!--wolf-->
```
let secret_number = 5

let guess = fn [guess : num]
	if guess > secret_number then "Too high..." 
	else if guess < secret_number then "Too low..." 
	else "Just right!"
```