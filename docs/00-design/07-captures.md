---
layout: page
title: Captures
page_number: 7
---

Captures describe how to name parts of a datum, so that the parts of the datum
can be used individually in other computations.

## Name captures

As seen before, the simplest capture is a name on its own.

In this case, the name refers to the datum as a whole.

```
-- `name_capture` refers to `4`.
let name_capture := 4
```

If desired, a type can be specified after a forward slash `/`.

```
let name_capture / num := 4
```

let name_capture / num := 4

## Tuple captures

Tuple syntax can be used to deconstruct a tuple datum.

Names with dots `.` can be used to access labelled data, while unprefixed names
can be used to access automatically indexed data.

```
let [first, second, third] := [1, 2, 3]
let [.first_name, .age] := [.first_name "Adam", .age 27]
```

Forward slashes can be used on individul names to specify types.

```
let [first / num, second / num, third / num] := [1, 2, 3]
let [.first_name / string, .age / num] := [.first_name "Adam", .age 27]
```

Forward slashes can also be used on a whole tuple capture.

```
let [first, second, third] / [num, num, num] := [1, 2, 3]
```

The rest of a tuple's data can be captured at the end using `...`.

```
-- `rest` becomes `[.age 27]`.
let [.first_name, ...rest] := [.first_name "Adam", .age 27]
```

## Vagueness

A capture that does not fully constrain the type it's expecting is called 
"vague".

```
-- These are vague captures, because there's no type constraint on the left hand
-- side of the assignment operator.
let cool_number := 42
let [first, second] := [2, 5]

-- These are not vague captures, because the types are fully constrained.
let cool_number / num := 42
let [first / num, second / num] = [2, 5]
```

When a vague capture is used, Wolf will attempt to fill in the vague areas with
inferred type information. This process only considers information that's
visible at the location where the capture is written.