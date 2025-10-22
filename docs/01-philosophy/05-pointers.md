---
layout: page
title: Pointers
page_number: 5
---

Pointers are a common feature of systems-level programming languages, and are used quite widely.
However, there are a few notable issues with pointers that motivate Wolf to avoid them.

Firstly, pointers are an implicit data dependency which do not need to be declared.
This is a leaky abstraction as the surrounding application still needs to consider this data dependency, but it is not declared anywhere.

Consider this hypothetical example of an array of employees, and a function returning a pointer/reference to a "receptionist":

```wf
let person = ty [
	.name : str
	.age  : num
]

let employees = [
	new person [
		.name "James"
		.age  35
	],
	new person [
		.name "Susan"
		.age  34
	],
	new person [
		.name "Mary"
		.age 38
	]
]
```

If Wolf had pointers or first-class references, it'd be possible to have a reference to a member of `employees` without _also_ having a reference to `employees` itself.
For example:

```wf
-- This function doesn't know that it will depend on `employees` specifically;
-- they can't even see the data source they depend on.
let get_employee_name = fn [employee: ^person] employee.name
```

Implicit data dependencies like these are a non-local effect that is hard to reason about, so Wolf decides to avoid them.
Wolf's stance is that the _concept_ of pointer-like handles are useful, but not implicit handles like these.

So, as an alternative, Wolf replaces these with index-like handles, which carry a number of advantages:

- Indices aren't usable without the data source being explicitly present.
- Indices tend to be smaller than the word size of the computer, saving memory and cache space.
- Index use is trivial to bounds-check and generation-check.
- Cases where index lookups fail can be more explicitly handled in code.

Wolf retains the use of pointers/references, but only as a second-class construct to allow for passing data sources.

To rework the above example:

```wf
-- The data dependency has been made clear.
-- A reference to the data source must be directly available, preserving local reasoning.
let get_employee_name = fn [employees: ^[... person], employee_id: num] (
	let employee = employees => select employee_id
	employee.name
)
```

In this way, the data source becomes a viral annotation on all code that requires access to it.
In turn, this allows data sources to be lexically scoped and served via requests.

```wf
let employees = ty [... person]

-- No need to explicitly bundle the data source with the input datum.
let get_employee_name = fn [employee_id: num] (
	let employee = req ^employees => select employee_id
	employee.name
)

let main = fn [] (
	let employees = new employees [ --- omitted for brevity --- ]
	prov ^employees (
		let receptionist_id = 2
		-- No need to explicitly bundle the data source into the input datum.
		let receptionist_name = get_employee_name [receptionist_id]
		receptionist_name
	)
)
```

This achieves two important goals:

- To the user, the data dependency is _quiet_ due to the requests mechanism.
- To the compiler, the data dependency is _explicit_ and lexically scoped.