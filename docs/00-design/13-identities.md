---
layout: page
title: Identities
page_number: 13
---

Identities are a special opaque data type in Wolf. They are never equal to
anything else, even other identities; they only ever equal themselves.

## Basic use

An identity is declared in any expression using the `identity` keyword.

```
let snowflake := identity
```

Identities from the same location in the source file are equal.

```
let fn get_identity [] := identity

-- `thing_1` will always equal `thing_2`, because their `identity` comes from
-- the same place.
let thing_1 := get_identity []
let thing_2 := get_identity []
```

Inversely, identities from different locations are not equal.

```
let fn get_identity_1 [] := identity
let fn get_identity_2 [] := identity

-- `thing_1` will never equal `thing_2`, because their `identity` comes from two
-- different places.
let thing_1 := get_identity_1 []
let thing_2 := get_identity_2 []
```

## Unique identities

To obtain a completely unique identity that won't appear again in the program,
call the `unique_identity` algorithm. This is only available in `algo` blocks.

```
let algo get_identity [] := unique_identity []

-- `thing_1` will never equal `thing_2`, even though `unique_identity` appears
-- in the same place, because `unique_identity` always returns a new identity
-- that has never been seen before in the program.
let thing_1 := get_identity []
let thing_2 := get_identity []
```

Conceptually, you can imagine `unique_identity` like a function that selects
from an infinite tuple of identities based on a counter that statefully
increments with each call.