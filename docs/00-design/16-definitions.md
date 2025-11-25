---
layout: page
title: Definitions
page_number: 16
---

Wolf allows you to define new types of data composed from existing types of data.

## Basic use

A type definition value is formed of two parts:

- The `ty` keyword.
- A capture or existing data type that forms the definition.

```
let person = ty [
	.name : str
	.age  : num
]

let catchphrase = ty str
```

Any captures in the same block or a nested block can use the defined type by name.

```
let speak = fn [
	.be  self   : person
	.say phrase : catchphrase
] ["Hello, my name is ", self.name, " and I love to say ", phrase]
```

## Uniqueness

Newly defined types are unique in Wolf's type system. The original capture and
the defined type are not interchangeable.

```
let name = ty str

let accept_string = fn [s : str] -- ... something ...
let accept_name = fn [s : name] -- ... something ...

-- This is allowed.
accept_string ["Hello"]

-- This is not allowed.
accept_name ["Hello"]
```

Instead, create a new instance of the type using the `new` keyword, followed by
the type name, followed by a value that matches the original capture.

```
-- This is allowed.
accept_name [new name "Hello"]
```

## First-class types

Type definitions in Wolf are first-class. 
That means they're values like any other your program deals with, and can be passed around to other code.

The `ty` type can be used to pass type definitions around.

```
let vector_type_of = fn [element : ty] [
	.vec2 ty [.x : element, .y : element]
	.vec3 ty [.x : element, .y : element, .z : element]
	.vec4 ty [.x : element, .y : element, .z : element, .w : element]
]
let [.vec3] = vector_type_of [num]
let foo : vec3 = new vec3 [.x 4, .y 25.5, .z -16]
```