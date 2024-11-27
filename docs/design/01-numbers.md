---
layout: page
title: Numbers
page_number: 1
---

The first basic data type in Wolf is `num` - a 64-bit IEEE floating point
number, which can exactly represent integers up to 2^53.


## Number literals

Numbers can be literally written with ASCII digits.

A dot may be added to specify a fractional part. This can't appear at the start
or end of the number.

Underscores can be included to visually split up the number, but they don't
change the final value.

```
10
1_337
6.21
```

## Special bases

Numbers can start with `0x` to use base 16, or `0b` to use base 2. Number bases
are case insensitive, as are the digits.

```
0X016aF2
0xBeefDadd
0b1001_0101
```

## Number keywords

The `inf` keyword is reserved to represent the largest possible number, and the
`nan` keyword is reserved to represent the "not a number" state.