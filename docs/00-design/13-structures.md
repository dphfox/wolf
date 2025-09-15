---
layout: page
title: Structures
page_number: 13
---

By default, data stored in a tuple is unstructured, meaning there's no way for
other parts of the codebase to know about the tuple's contents in advance.

Wolf allows tuples to store structured sections of data, so that it can be
shared between different parts of the codebase.

## Declaring a structure

A structure can be declared with the `struct` keyword, followed by brackets `[]`.
Inside the brackets, tuple capture syntax can be used to declare what data is
expected to be present.

```
let date := struct [
	.year / num
	.month / num
	.day / num
]

let person := struct [
	.name / str
	.is_verified / bool
]
```

## Structured sections

Tuples can store multiple sections of structured data. The name of the section
refers to a structure somewhere in the namespace.

To start a structured section, write the name of the section, followed by a
colon `:`. 

```
let example_user := [
	.internal_id 12345

	person:
	.name "Jane Doe"
	.is_verified true

	date:
	.year 1995
	.month 7
	.day 16
]
```

## Namespacing

Each section has its own namespace, allowing multiple structures to have data
with the same name.

```
-- This is valid.
let foo := struct [ value / num ]
let bar := struct [ value / str ]
let combined_data := [
	foo: .value 5
	bar: .value "Hello!"
]
```

## Accessing structured data

If only one section uses a name for a datum, the datum can be accessed directly.

```
let the_year = example_user.year
let the_name = example_user.name
```

Otherwise, the explicit syntax can be used, where the section name is prepended
to the datum name with a colon `:`.

```
let the_year = example_user.date:year
let the_name = example_user.person:name
```

## Capturing data from sections

Tuple captures follow the same rules as tuple accesses.

```
let [.year, .name] = example_user
let [.date:year, .person:name] = example_user
```

## Rest-of-section capture

A rest-of-tuple capture can be converted to a rest-of-section capture by
prepending the name with the section to be captured.

```
let [...date:the_date] = example_user
```

If the names match, the second name can be omitted.

```
-- These two statements are identical.
let [...date:date] = example_user
let [...date:] = example_user
```

Multiple rest-of-section captures can be combined, and can be coupled with a 
rest-of-tuple capture.

```
let [...date:, ...person:, ...rest] = example_user
```