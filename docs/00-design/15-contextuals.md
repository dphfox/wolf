---
layout: page
title: Contextuals
page_number: 15
---

Contextuals are identifiers which can have multiple different definitions, 
based on the current context they're in.

This is analogous to "generics" or "compile-time values" found in other
languages.

## Contextual lets

Any `let` declaration can be made contextual by adding the `context` keyword
to it.

This allows other code to redefine the value.

```
let context my_thing := 25
```

## With context

In any block, you may use a `with` declaration to redefine any contextual
that's visible from your current location.

The new value must pass the same type constraints as the original value.

The redefinition is only visible within the current block.

```
let context contextual_value := 25
let double_the_value := fn [] contextual_value * 2

let ten := (
	-- Any calculations (directly or indirectly) inside the block will see the
	-- redefined `contextual_value`.
	with contextual_value := 5 
	double_the_value []
)

-- Calculations outside of the block will not see the redefinition.
let fifty := double_the_value []
```

## Required contextuals

For contextual lets, the assignment can be omitted, as long as the capture isn't
vague.

The contextual's value must be defined using a `with` declaration before its
value can be used. This is known as a _required contextual_.

```
-- The type is required to make this capture non-vague.
let context my_thing / num
```