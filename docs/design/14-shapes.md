---
layout: page
title: Shapes
page_number: 14
---

All data in Wolf has a "shape" determined at compile time. This shape describes
all the information known about the data at compile time.

## Construction

Shapes are defined in angle brackets `<` `>`.

Numbers, strings and uniques can be used as basic shapes.

```
<25>
<"foo">,
<unique>
```

Const operations can also be included; the shape of the output is determined at compile time.

```
<2 + 2>
<"foo" :concat "bar">
<!false>
```



## Vague shapes

Sometimes the exact shape of the data can't be known at compile time, for example because it comes from user input.
For these cases, Wolf provides "vague" shapes whose exact shape isn't known until runtime.

Underscores `_` are inferred to be the vaguest possible valid shape.
You can use operators to whittle down the possibility space.

```
<_> -- any possible shape
<_ :concat _> -- the shape you get by concatenating two things
<2 + _> -- the shape you get by adding something to a number
```

Additionally, Wolf provides builtin `num` and `str` vague types that match the valid values of each kind of data.

```
<num> -- any possible f64 number
<str> -- any possible utf8 string
```

