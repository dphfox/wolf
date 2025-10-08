---
layout: page
title: Memory
page_number: 15
---

Wolf uses a straightforward model for managing memory in a compiler-checked way.

## Values in blocks

When a value is captured for the first time, the function stores the value in a
designated memory location.

When the values are next captured, the memory location is referenced instead of
copying the value.

At the end of the block, the returned value is moved out of the block's memory.
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
replacing them with in-place operations.