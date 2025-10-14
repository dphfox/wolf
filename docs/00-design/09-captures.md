---
layout: page
title: Captures
page_number: 9
---

Captures describe how to name parts of a datum, so that the parts of the datum can be used individually in other computations.

## Name captures

As seen before, the simplest capture is a name on its own.

In this case, the name refers to the datum as a whole.

<!--wolf-->
```
-- `name_capture` refers to `4`.
let name_capture = 4
```

If desired, a type can be specified after a colon `:`.

<!--wolf-->
```
let name_capture : num = 4
```

## Tuple captures

Tuple syntax can be used to deconstruct a tuple datum into a set of names.

<!--wolf-->
```
let [first, second, third] = [1, 2, 3]
```

### Explicit names

Explicitly named data can be accessed by including a dot-prefixed name before the name you wish to capture into.

<!--wolf-->
```
let [.first_name first_name, .age age] = [.first_name "Adam", .age 27]
```

If the two names are the same, the second name can be omitted.

<!--wolf-->
```
let [.first_name, .age] = [.first_name "Adam", .age 27]
```

### Tuple typing

Colons can be used on individul names to specify types.

<!--wolf-->
```
let [first : num, second : num, third : num] = [1, 2, 3]
let [.first_name : string, .age : num] = [.first_name "Adam", .age 27]
```

Colons can also be used on a whole tuple capture to type the whole tuple at once.

<!--wolf-->
```
let [first, second, third] : [num, num, num] = [1, 2, 3]
```

### Rest-of-tuple capture

Mirroring tuple flattening syntax, the rest of a tuple's data can be captured at the end using ellipsis `...`.

<!--wolf-->
```
-- `rest` becomes `[.age 27]`.
let [.first_name, ... rest] = [.first_name "Adam", .age 27]
```

Multiple rest-of-tuple captures are not allowed because it would be ambiguous how much to capture for each one.

<!--wolf-->
```
-- This is not allowed.
let [... one, ... two] = [.first_name "Adam", .age 27]
```