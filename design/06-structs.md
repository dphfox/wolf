# Structs

Wolf allows the basic types of data to be *composed* into structured data types,
known as *structs*.

Structs are static, immutable collections of data.

## Empty structs

An empty struct can be declared with curly braces `{}`.

```
empty := {}
```

## Named data

Structs can contain identifier expressions to save named data inside them,
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

## Indexed data

If a struct doesn't contain any named data, it can contain *indexed* data
instead. Each field is implicitly given an index based on its position in the
struct, starting at `1`.

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
cool_year := cool_date.1
```