---
layout: page
title: Throwing
page_number: 14
---

Sometimes, deeply nested expressions need to exit early from a computation, or pass a value directly back up to a higher block without further processing.

Wolf implements a non-viral, strictly-typed "throwing" and "catching" mechanism to allow nested expressions to exit early.

## Catching

A block can be prefixed with `catch`. This indicates that the block will catch any value thrown to it.
When that happens, the block will evaluate to that thrown value.

<!--wolf-->
```
let can_catch = catch (2)
```

`catch` does not change what the block returns by default.

<!--wolf-->
```
-- These two expressions are identical.
let foo = (2)
let foo = catch (2)
```

## Throwing

While inside of a `catch` block, any expression can start with `throw`. 
This short-circuits the rest of the computation and throws the value to the nearest ancestor `catch` block.

<!--wolf-->
```
let three = catch (
	-- 4 and 5 are not evaluated.
	1 -> 2 -> throw 3 -> 4 -> 5
)
```

Throwing is strictly local; you cannot throw to a `catch` block elsewhere in the source code.
In particular, the call stack of the function is not considered.

<!--wolf-->
```
-- This is not allowed.
let foo = fn [message : str] throw message

let bar = catch ( foo ["Hello world"] )
```

Even if the whole call stack is locally known, it is not considered at all.
Throws are explicitly _lexically scoped_.

<!--wolf-->
```
-- `bar` becomes "Hello, world".
let bar = catch (
	-- This `throw` will go straight to bar's `catch` immediately.
	let foo = fn [message : str] throw message

	-- garb's `catch` block can only be discovered via the call stack, so it's not considered. 
	let garb = catch (
		foo ["Hello, world"] -- immediately throws to `bar`
	)
	-- This value will never be returned as a result.
	"Goodbye, world"
)
```