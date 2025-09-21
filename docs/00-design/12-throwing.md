---
layout: page
title: Throwing
page_number: 12
---

Sometimes, deeply nested expressions need to exit early from a computation, or
pass a value directly back up to a higher block without further processing.

Wolf implements a non-viral, strictly-typed "throwing" and "catching" mechanism
to allow nested expressions to exit early.

## Catching

A block can be prefixed with `catch`. This indicates that the block will catch
any value thrown to it. When that happens, the block will evaluate to that 
thrown value.

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

While inside of a `catch` block, any expression can start with `throw`. This
short-circuits the rest of the computation and throws the value to the nearest
ancestor `catch` block.

<!--wolf-->
```
let three = catch (
	-- 4 and 5 are not evaluated.
	1 -> 2 -> throw 3 -> 4 -> 5
)
```

Throwing is strictly local; you cannot throw to a `catch` block elsewhere in the
source code.

<!--wolf-->
```
-- This is not allowed.
let foo = fn [message : str] throw message

let bar = catch ( foo ["Hello world"] )
```

All thrown values must evaluate to a type compatible with the `catch` block.

<!--wolf-->
```
-- This is not allowed.
let foo = catch ( if true then throw "string" else 5 )
```