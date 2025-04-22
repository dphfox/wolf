---
layout: page
title: Captures
page_number: 7
---

Captures describe how to name parts of a datum.

## Names

As seen before, the simplest capture is a name on its own.

In this case, the name refers to the datum itself.

```
-- `name` refers to `4`.
let name: 4
```

## Tuples

Instead of a name, tuple syntax can be used to deconstruct a tuple datum.

Names can be specified in place of data.

```
-- Simple tuple capture.
let [first, second, third]: [1, 2, 3]

-- Named tuple capture.
let [first_name: first_name, age: age]: [first_name: "Adam", age: 27]
```

When capturing named tuples, if the name before and after the colon is the same,
the second name can be omitted.

```
-- These two captures are equivalent.
let [first_name: first_name, age: age]: [first_name: "Adam", age: 27]
let [:first_name, :age]: [first_name: "Adam", age: 27]
```

The rest of a tuple's data can be captured at the end using `...`.

```
-- `rest` becomes `[age: 27]`.
let [:first_name, ...rest]: [first_name: "Adam", age: 27]
```