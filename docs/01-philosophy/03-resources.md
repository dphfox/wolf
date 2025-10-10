---
layout: page
title: Resources
page_number: 3
---

In Wolf, resource management follows a simple rule: you can't create something
without locally knowing who will destroy it. This rule was chosen because it
can be explained in one sentence, and ensures users of Wolf can always apply
local reasoning when understanding what their programs do.

Applying this rule leads to a simple variant of region-based memory management.

Consider the common example of memory management. Suppose we define a variable
inside of a block, which stores a value that requires allocation.

<!--wolf-->
```
(
	let needs_memory = [.name "steve", .age 27]
)
```

We need to locally know when `needs_memory` will be freed. This is trivially
knowable because blocks are lexically scoped; when the block exits, the memory
will never be used again. So, we can locally infer how long `needs_memory` needs
to exist. So, we can allocate memory here, knowing that we can handle its
destruction.

<!--wolf-->
```
-- Pseudocode
(
	let local_alloc = allocator
	let needs_memory = malloc [.name "steve", .age 27] in local_alloc
	free local_alloc
)
```

To complicate the example, let's now pass the value out of the block. It is now
invalid to free `needs_memory` at the end of the block, because it can now be
used after the block exits. So, we do not locally know who will destroy it.

<!--wolf-->
```
-- Pseudocode
(
	let local_alloc = allocator
	let needs_memory = malloc [.name "steve", .age 27] in local_alloc
	let will_return_this = malloc [.name "baxter", .age 5] in local_alloc
	free local_alloc -> will_return_this -- invalid
)
```

To solve this, let's reimagine blocks like they're functions, which take in
some kind of "return" allocator. This lets us allocate any final values into
a separate allocator that won't be freed when the block exits, while keeping all
reasoning local.

<!--wolf-->
```
-- Pseudocode
fn [.return_alloc : allocator] (
	let local_alloc = allocator
	let needs_memory = malloc [.name "steve", .age 27] in local_alloc
	-- Important: use the return allocator here
	let will_return_this = malloc [.name "baxter", .age 5] in return_alloc
	free local_alloc -> will_return_this -- Now OK
)
```

With this understanding, we can recursively assemble any blocks together to form
any program we want, and it's guaranteed that all allocated memory will be freed
exactly once, after it is no longer accessible.

This technique can then be used to implement higher level resource management
primitives like arena allocators while still preserving safety guarantees.