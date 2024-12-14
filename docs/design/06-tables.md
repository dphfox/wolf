---
layout: page
title: Tables
page_number: 6
---

Wolf allows the basic types of data to be composed together using *tables*.

Tables are made of key/value pairs. The value can be retrieved if you know the
key.

## Empty tables

An empty table can be declared with curly braces `{}`.

```
empty := {}
```

## Named pairs

Tables can contain identifier expressions to save named data inside them,
separated by commas `,`.

```
cool_date := { year := 2015, month := 05, day := 15 }
```

Similarly to blocks, Wolf will automatically insert commas when starting a new
expression on the next line.

```
cool_date := {
	year := 2015
	month := 05
	day := 15
}
```

Data can be extracted using a dot `.` followed by the name of the identifier.

```
cool_year := cool_date.year
```

## Numbered pairs

Tables can contain *implicitly numbered* data instead. Each value is given a
key based on its position in the struct, starting at `0`.

Expressions are listed directly, rather than with identifiers.

```
cool_date := { 2015, 05, 15 }

cool_date := {
	2015
	05
	15
}
```

Data can be extracted using a dot `.`, followed by the index given as a number
literal.

```
cool_year := cool_date.0
```

## Select data via expression

Expressions can be used to extract data from tables.

To access named data, use `.()`, with a `str` expression between the
parentheses.

```
cool_date.("year")
```

To access numbered data, use `.()` with a `num`
expression between the parentheses.

```
cool_date.(2)
```

## Count of data

Use the count `#` operator on any table to count the number of data fields it
contains.

```
$ wf -- #{ year := 2015, month := 05, day := 15 }
3

$ wf -- #{}
0

$ wf -- #{1, 2, 4, 8, 16}
5
```