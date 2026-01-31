---
layout: page
title: Pointers
page_number: 5
---

First class pointers are a common feature of systems-level programming languages, and are used quite widely.
However, first class pointers hide an implicit data dependency which makes them awkward and unfriendly to use.

## Pointer problems

Consider this hypothetical example of an array of employees being created by a function that returns a standard pointer:

<!--wolf-->
```
let person = ty (
	.name : str
	.age  : num
)

let create_employees = fn () (
	let employees = (
		new person (
			.name "James"
			.age  35
		),
		new person (
			.name "Susan"
			.age  34
		),
		new person (
			.name "Mary"
			.age 38
		)
	)

	^employees -- Pseudocode; return a pointer to the employees
)

let main = fn () (
	let employees = create_employees ()
)
```

The pointer returned from `create_employees` has an implicit dependency on `employees`, so you can only have a valid pointer while `employees` is alive.
So, there are three main options for this scenario:
- Force the data to keep living as long as there are live pointers to it
- Allow pointers to outlive data, but error at runtime if you use a pointer that outlives the data
- Do not allow pointers to outlive the data

The first two options are undesirable for Wolf's local reasoning goals:

- By making the data live arbitrarily longer if there are _any_ pointers to it, you break local reasoning about the data, because non-local code can make data live arbitrarily long.
- By erroring at runtime, you introduce new implicit, dynamic and unpredictable control flow into any code that uses a pointer, even transitively, breaking local reasoning.

Only the third option preserves local reasoning properly.

## The pointer rule

That's why Wolf implements the _pointer rule_: you can only hold a pointer to data while the data is in scope.

Let's annotate why the above example fails:

<!--wolf-->
```
let create_employees = fn () (
	-- This data is accessible anywhere in the tuple of `create_employees`.
	let employees = ( --- omitted --- )
	-- It is valid to create a pointer to this data, because we can see it here.
	let pointer_to_employees = ^employees
	-- It is _not_ valid to pass this pointer out of this tuple, because the
	-- scope of the data ends here.
	pointer_to_employees
)
```

## Pointer techniques

To make the example work, we _could_ return both the employees and the pointer, and it would become valid again because
the data moves to the outer scope alongside the pointer.

<!--wolf-->
```
let create_employees = fn () (
	let employees = ( --- omitted --- )
	(.data employees, .pointer ^employees) -- This is OK again.
)

let main = fn () (
	-- The data re-appears here next to the pointer; everything works.
	let (.data, .pointer) = create_employees ()
)
```

A notable consequence of this is that functions can still _accept_ pointers, because the data comes from the outer scope.
Any derived pointers from those inputs can also still be returned from functions, because the underlying data is the
same.

<!--wolf-->
```
let point_at_someone = fn (employees : ^(... person) (
	employees.2 -- take the `employees` pointer and derive a new pointer to the third employee
)

let main = fn () (
	-- The data exists in this outer scope.
	let employees = ( --- omitted --- )
	-- We create a pointer to the data while it's still in this outer scope.
	let pointer_to_employee = point_at_someone (^employees)
)
```

This even works for requests, because those are also scoped.
This allows for compiler-checked transparent passing of data sources to inner functions.

<!--wolf-->
```
let employees = ty ^(... person)

let point_at_someone = fn () (
	-- Request the data for use in the inner scope.
	let employees = req employees
	employees.2
)

let main = fn () (
	-- Provide the data in the outer scope.
	prov new employees ^( --- omitted --- ) (
		let pointer_to_employee = point_at_someone ()
	)
)
```