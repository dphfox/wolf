---
layout: page
title: Trying
page_number: 18
---

Wolf's type system ensures all data processed by a program meets a certain set of restrictions.
By default, inputs that don't meet those restrictions will cause a compiler error.

Instead of erroring, `try` statements allow code to explicitly handle data that does not meet those restrictions.

## Basic use

`try` expressions are formed like `if` expressions:

- The `try` keyword.
- An "attempt" expression that might not type check.
- The `else` keyword. 
- A default expression to be used when the attempt expression does not type check.

<!--wolf-->
```
-- If `user_config` has a type which can satisfy this expression, then it will be run.
-- Otherwise, the default value of 1000 will be used.
let timeout = try user_config.service.timeout else 1000
```

## Multiple expressions

Multiple `try` sections can be added, similarly to an `if` expression, to define a series of fallback expressions.

<!--wolf-->
```
-- The expressions are attempted in order of appearance.
let timeout = 
	try workspace_config.service.timeout
	try folder_config.service.timeout
	try user_config.service.timeout
	else 1000
```