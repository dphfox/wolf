---
layout: page
title: Tuples
page_number: 3
---

In Wolf, tuples group multiple data into a single datum.


## Construction

Tuples are constructed by enclosing data in square brackets `[ ]`.
Data is separated by either commas `,` or newlines.

```
[1, 2, 3]

[
	1, 2, 3
	4, 5, 6
	7, 8, 9
]
```

Empty tuples are also allowed. Conceptually, they contain no data.

```
[]
```

## Single-value tuples

Wolf can automatically convert between single-value tuples of any depth. This
includes getting rid of all tuples.

```
2
[2]
[[2]]
[[[2]]]
```

This is only done in limited situations where there's a clear conversion to be
made.

## Labelled data

By default, data in tuples is indexed automatically by position, starting at 0
for the first datum, 1 for the second datum, etc.

You can explicitly label the indices by putting the index before the data, with
a dot `.` prefix. You can use this to skip indices.

```
[
	.10 2015, 
	.20 5, 
	.30 15
]
```

Negative or fractional indices cannot be specified.

```
-- This is not allowed.
[
	.-1 2015
	.0.5 5
	.-0 15
]
```

Labels can't be reused in the same tuple.

```
-- This is not allowed.
[
	.0 2015, 
	.0 5, 
	.0 15
]
```

You may also use an alphanumeric name.

```
[
	.year 2015
	.month 5
	.day 15
]
```

Names are written as an alphanumeric string, plus underscores. They have the
following limitations:

- Reserved keywords can't be used in names.
- Names that look like number values are disallowed.
- Underscores can't be used at the start or end of a name.

```
-- These names are valid.
[
	.person 1
	.PEOPLE 2
	.Crowd 3
	.large_gathering 4
	.smallGroup 5
	.3rd_Person 6
]
```

Conventionally, names are written in `snake_case`.

It's valid to mix labelled and unlabelled data; unlabelled data will not 
consider labelled data when determining indices.

```
[
	1
	2
	.foo "a"
	3,
	.bar "b",
	4,
	5
]
```

## Accessing data

You can access a datum with the dot `.` operator, followed by the index or label
of the datum you want to take.

```
-- Evaluates to 5.
[3, 5, 7].1

-- Evaluates to 2015.
[.year 2015, .month 5, .day 15].year
```

## Flattening

If you're putting a tuple inside of another tuple, you can flatten its contents
by using an ellipsis `...` after it.

```
-- These two expressions are equivalent.
[[1, 2, 3]..., 4, 5]
[1, 2, 3, 4, 5]

-- So are these two expressions.
[[.year 2015, .month 5]..., .day 15]
[.year 2015, .month 5, .day 15]
```