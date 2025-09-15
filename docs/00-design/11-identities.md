---
layout: page
title: Identities
page_number: 11
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
let get_identity := fn [] identity

-- `thing_1` will always equal `thing_2`, because their `identity` comes from
-- the same place.
let thing_1 := get_identity []
let thing_2 := get_identity []
```

Inversely, identities from different locations are not equal.

```
let get_identity_1 := fn [] identity
let get_identity_2 := fn [] identity

-- `thing_1` will never equal `thing_2`, because their `identity` comes from two
-- different places.
let thing_1 := get_identity_1 []
let thing_2 := get_identity_2 []
```