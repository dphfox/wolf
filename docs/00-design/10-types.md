---
layout: page
title: Types
page_number: 10
---

Types annotations restrict a datum without capturing it into a name. 
They appear after the colon `:` in a capture.

## Type names

Wolf provides a few basic type names, some of which you've seen already:

- `num` - floating point numbers
- `int` - integers
- `uni` - the unique value - only equal to itself
- `bool` - boolean outcomes
- `str` - strings of characters
- `ty` - a type definition

## Tuple types

As with captures, tuple syntax can be used to represent the type of various
tuples.

In place of sub-values or sub-captures, sub-types are provided as entries.

```
(num, str, bool)
(.name str, .age num)
```

As shorthand for arrays with known quantities of a single type, write the
quantity as an integer, followed by the type.

```
-- These are equivalent.
(num, num, num)
(3 num)

-- These are also equivalent.
(str, bool, bool)
(str, 2 bool)
```

## Rest-of-tuple types

The rest of a tuple's data can be constrained to a certain type using ellipses `...`.
This includes both named and unnamed data.

This is most commonly used to represent dynamically-sized arrays.

```
-- Any quantity of strings.
(... str)

-- A name, an age, and some booleans.
(.name str, .age num, ... bool)
```