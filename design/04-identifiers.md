# Identifier expressions

Identifiers allow the value of an expression to be referred to.

Identifiers are written as an alphanumeric ASCII string (+ underscores),
followed by `:=` and the expression.

Identifiers are scoped to the current block, if any. Otherwise, they're added
to the top-level namespace.

Identifiers do not care about the order of appearance.

```
(
	sixteen := four * four
	four := 2 + 2
	sixteen - four
)
```

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

Identifier expressions evaluate to the same value as the sub-expression they
refer to.

```
sixteen := (four := 2 + 2) ^ 2
```

By default, the data type of the identifier is inferred from the expression.
However, an explicit type can be named between the `:` and `=`.

```
four: num = 2 + 2
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

Identifiers cannot form infinite cycles. The following would *not* be allowed:

```
(
	two := four / 2
	four := two * 2
)
```