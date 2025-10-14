---
layout: page
title: Tuples
page_number: 5
---

In Wolf, tuples group multiple data into a single datum.

## Construction

Tuples are constructed by enclosing data in square brackets `[ ]`.
Data is separated by either commas `,` or newlines.

<!--wolf-->
```
["hello", "world", 2025]

[
	1, 2, 3
	4, 5, 6
	7, 8, 9
]
```

Empty tuples are also allowed. Conceptually, they contain no data.

<!--wolf-->
```
[]
```

## Single-value tuples

Wolf can automatically convert between single-value tuples of any depth. 
This includes getting rid of all tuples.

<!--wolf-->
```
2
[2]
[[2]]
[[[2]]]
```

This is only done in limited situations where there's a clear conversion to be made.

## Named data

By default, data in tuples is named automatically by position, starting with `0` for the first datum, `1` for the second datum, etc.

<!--wolf-->
```
[
	2015 -- named `0`
	5    -- named `1`
	15   -- named `2`
]
```

You can explicitly provide names by putting a dot-prefixed name before the data.
As with all names, backticks are optional.

<!--wolf-->
```
[
	.`year` 2015 -- named `year`
	.month  5    -- named `month`
	.day    15   -- named `day`
]
```

Names can't be reused in the same tuple.

<!--wolf-->
```
-- This is not allowed.
[
	.`0` 2015, 
	.0   5, 
	.0   15
]
```

It's valid to mix explicitly named data with unnamed data. 
Unnamed data will not consider explicitly named data when assigning automatic names.

Be aware that explicitly named data can still have namespace collisions with automatically named data.

<!--wolf-->
```
[
	0         -- named `0`
	1         -- named `1`
	.foo "a"  -- named `foo`
	2,        -- named `2`
	.bar "b", -- named `bar`
	3,        -- named `3`
	4         -- named `4`
]
```

## Accessing data

You can access a datum with the dot `.` operator, followed by the name of the datum to be accessed.

<!--wolf-->
```
-- Evaluates to 5.
[3, 5, 7].1

-- Evaluates to 2015.
[.year 2015, .month 5, .day 15].year

-- Evaluates to "Bob"
[.name "Bob", .age 25].`name`
```

## Flattening

If you're putting a tuple inside of another tuple, you can flatten its contents by using an ellipsis `...` instead of a name.

<!--wolf-->
```
-- These two expressions are equivalent.
[... [1, 2, 3], 4, 5]
[1, 2, 3, 4, 5]

-- So are these two expressions.
[... [.year 2015, .month 5], .day 15]
[.year 2015, .month 5, .day 15]
```