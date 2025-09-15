---
layout: page
title: Captures
page_number: 9
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

## Tuple captures

Tuple syntax can be used to deconstruct a tuple datum into a set of names.

```
let [first, second, third] := [1, 2, 3]
```

### Explicit names

Explicitly naed data can be accessed by including a dot-prefixed name before the
name you wish to capture into.

```
let [.first_name first_name, .age age] := [.first_name "Adam", .age 27]
```

If the two names are the same, the second name can be omitted.

```
let [.first_name, .age] := [.first_name "Adam", .age 27]
```

### Tuple typing

Forward slashes can be used on individul names to specify types.

```
let [first / num, second / num, third / num] := [1, 2, 3]
let [.first_name / string, .age / num] := [.first_name "Adam", .age 27]
```

Forward slashes can also be used on a whole tuple capture.

```
let [first, second, third] / [num, num, num] := [1, 2, 3]
```

### Rest-of-tuple capture

The rest of a tuple's data can be captured at the end using `...`.

```
-- `rest` becomes `[.age 27]`.
let [.first_name, ...rest] := [.first_name "Adam", .age 27]
```