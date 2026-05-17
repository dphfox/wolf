---
layout: page
title: Views
page_number: 18
---

On occasion, it can be useful for non-local code to provide a new value for a locally-declared value.

Views implement this in a safe, lexically-scoped way.

## Basic use

A view is created by wrapping a declared name in curly braces `{ }`.

<!--wolf-->
```
let total : num = 5
let total_view : {num} = {total}
```

Views cannot be passed out of the scope where the original value is defined, unless the original value is passed out at the same time.

<!--wolf-->
```
-- This is not allowed.
let total_view = (
	let total = 5
	{total}
)

-- This is allowed.
let (total, total_view) = (
	let total = 5
	(total, {total})
)
```

Once a view has been created, it can be be used to shadow the original declaration via captures.

<!--wolf-->
```
let increase = fn(
	.by delta : num
	.total : {num}
	.count : {num}
) (
	let {total} = total + delta
	let {count} = count + 1
	()
)

let my_total = 0
let my_count = 0

let () = increase(.by 1, .total {my_total}, .count {my_count})
let () = increase(.by 5, .total {my_total}, .count {my_count})
let () = increase(.by 2, .total {my_total}, .count {my_count})

-- my_total is now 8, my_count is now 3
```

Note that this is not the same as _mutation_ - it explicitly creates a new declaration for the viewed value.

<!--wolf-->
```
let bump = fn(.x : {num}) (
	let {x} = x + 1
	()
)

let counter = 0
let original_view = {counter}

----
This bump() creates a new `counter` declaration and assigns it the new value.
However, `original_view` still views the previous `counter`.
This lets you shadow a value without losing access to the old value in the process.
----
let () = bump(original_view) -- `let counter = 0 + 1`
let () = bump(original_view) -- still `let counter = 0 + 1`
let () = bump(original_view) -- still `let counter = 0 + 1`

---
You must create a new view if you want to continue updating the value cumulatively.
---
let () = bump({counter}) -- `let counter = 1 + 1`
let () = bump({counter}) -- `let counter = 2 + 1`
let () = bump({counter}) -- `let counter = 3 + 1`
```