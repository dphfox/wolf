---
layout: page
title: Memory
page_number: 16
---

Wolf uses a straightforward model for managing memory in a compiler-checked way.

## Basic use

When a value is captured for the first time, the function stores the value in a
designated memory location.

When the values are next captured, the memory location is referenced instead of
copying the value.

At the end of the block, the returned value is copied out of the block's memory.
Any remaining memory left in the block is freed.

<!--wolf-->
```
let the_name = (
	-- Start saving values into the block's memory...
	let person = [.name "Steve", .age 27] 
	-- Refer to parts of the memory from before...
	let [.name, .age] = person
	-- Move this slice of memory out to `the_name`, get rid of the rest
	name
)
```

This simple memory management scheme works because let declarations aren't
mutable by default; each declaration has exactly one assignment.

Note that Wolf will optimise away these memory operations where possible,
replacing them with cheaper options, or evaluating values at compile time.

## Views

TODO: expand on this

Views allow non-local code to directly reference definitions from inside of the
current block.

<!--wolf-->
```
let person = ty [ .name : str, .age : num ]

let get_name = fn [person : view person] person.name

let the_name = (
	let person = [.name "Steve", .age 27] 
	let name = get_name [view person]
	name
)
```

Views are only valid while the referenced definition is in scope. In particular,
this means functions can only return views derived from their inputs.

Views can collect together multiple sources.

<!--wolf-->
```
let subject = ty [ .name : str, .age : num ]
let human   = ty [ .catchphrase : str ]
let dog     = ty [ .howls : bool ]

let steve = view {
	subject [ .name "Steve", .age 27 ]
	human   [ .catchphrase "Cheerio!"]
}

let baxter = view {
	subject [ .name "Baxter", .age 5 ]
	dog     [ .howls true ]
}
```

## Functions

By default, functions take views to names that originate from the surrounding
code.

<!--wolf-->
```
-- `message` is of type `str` here
let message = "Hello"
let func_1 = fn [] (
	-- `message` is of type `view str` here
	let view_of_message = message
)
```

This means that functions are subject to the same rules as views by default,
unless they do not take any views to surrounding names.

For example, if a function uses data that only exists in the current block, it
can't be returned until that data is moved out to an accessible location.