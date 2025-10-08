---
layout: page
title: Types
page_number: 14
---

Wolf allows you to create aliases for commonly used data types.

## Basic use

In any expression, a type can be declared using the `ty` keyword, followed by
a capture or another existing type.

```
let person = ty [
	.name : str
	.age  : num
]

let catchphrase = ty str
```

Any captures in the same block or a nested block can use the type by name.

```
-- Custom types in let declarations.
let steve : person = [
	.name "Steve"
	.age  17
]

-- Using custom types in a function.
let speak = fn [
	.as  self   : person
	.say phrase : catchphrase
] ["Hello, my name is ", self.name, " and I love to say ", phrase]
```

## Structural types

Types in Wolf are _structural_ - that is to say, any type definition can be
swapped for its contents.

For example, since `catchphrase` is defined as `ty str`, we can just pass in a
string. You don't need to specifically make it a `catchphrase`.

```
let `steve's message` = speak [.as steve, .say "cheerio!"]
```

## First-class types

Types in Wolf are first-class. That means they're values like any other your
program deals with, and can be passed around to other code.

The `ty` type can be used to pass type definitions around.

```
let vector_type_of = fn [element : ty] [
	.vec2 ty [.x : element, .y : element]
	.vec3 ty [.x : element, .y : element, .z : element]
	.vec4 ty [.x : element, .y : element, .z : element, .w : element]
]
let [.vec3] = vector_type_of [num]
let foo : vec3 = [.x 4, .y 25.5, .z -16]
```