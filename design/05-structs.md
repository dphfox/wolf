# Structs

Wolf allows the basic types of data to be *composed* into structured data types,
known as *structs*.

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

## Explicit struct types

If needed, structs can be typed. The syntax is similar, but without any
expressions or equal `=` symbols.

Both named and indexed struct types can be written over multiple lines, with the
same automatic comma insertion rules for both.

```
cool_date : {
	year: num
	month: num
	day: num
} = { year := 2015, month := 05, day := 15 }

cool_date : {num, num, num} = { 2015, 05, 15 }
```