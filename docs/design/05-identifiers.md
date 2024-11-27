---
layout: page
title: Identifiers
page_number: 5
---

Identifiers allow the value of an expression to be referred to.

## Basic usage

Identifiers are written as an alphanumeric ASCII string (+ underscores),
followed by `:=` and the expression.

```
four := 2 + 2
```

Identifiers have the following limitations:

- Reserved keywords can't be used in identifiers.
- Identifiers that look like literal numbers are disallowed.
- Underscores can't be used at the start or end of an identifier.

```
person
PEOPLE
Crowd
large_gathering
smallGroup
3rd_Person
```

Identifier expressions evaluate to the same value as the sub-expression they
refer to.

```
sixteen := (four := 2 + 2) ^ 2
```

## Identifiers in blocks

Identifiers are scoped to the current block, if any. Otherwise, they're added
to the top-level namespace. Order of appearance is not considered.

```
(
	sixteen := four * four
	four := 2 + 2
	sixteen - four
)
```

Identifiers cannot form infinite cycles. The following would *not* be allowed:

```
(
	two := four / 2
	four := two * 2
)
```

## Shadowing

Identifiers are not mutable or reusable - duplicates are not allowed in the same
block.

However, they may be overshadowed by identifiers inside of a nested block.

```
(
	foo := 5
	(
		two_thousand := foo * 2
		foo := 1000
	)
	ten := foo * 2
)
```

Identifiers can be dropped with `:= _`, overshadowing any identifiers from
outside of the block without giving it a new value.

This ensures a block doesn't use a specific identifier, though nested blocks
may overshadow it again.

```
(
	foo := 5
	(
		foo := _
		(
			foo := 12
		)
	)
)
```