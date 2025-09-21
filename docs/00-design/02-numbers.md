---
layout: page
title: Numbers
page_number: 2
---

Numbers in Wolf are represented with 64-bit IEEE floating point numbers.

All names that look like numbers are reserved by Wolf, and refer to the floating
point value that minimally diverges from the written digits. Wolf programs
cannot redefine what these names mean.

## Integers

Names consisting of only digits refer to integer numbers.

<!--wolf-->
```
-- These both refer to the floating point value of 12345
`12345`
12345
```

All integers up to 2^53 can be exactly represented. Larger integers are stored
with precision loss.

## Underscores

Underscores `_` are permitted between digits, and will be ignored by Wolf.

<!--wolf-->
```
-- These both refer to the floating point value of 12345
1_2_3_4_5
`12_345`
```

## Reals

A decimal point `.` can be used in the name to delimit an integer and fractional
part.

If using backticked names, only one of the parts needs to be present.

<!--wolf-->
```
-- These refer to the floating point value of 0.125
0.125
`0.125`
`.125`

-- This refers to the floating point value of 256.0
`256.`
```

## Scientific notation

An exponent can be added to any number when using backticks.

Use `e` after the main part of the number to delimit the exponent. It must be 
followed by either `+` or `-` to denote the sign of the exponent. Then, write
the integer exponent.

Exponents are case sensitive and are only available when using decimal digits.

<!--wolf-->
```
-- These are valid exponents.
`1.23e+45`
`1.23e-45`
-- This is not a valid exponent.
`1.23e45`
```

## Change of base

Numbers starting with `0x` are interpreted using hexadecimal digits instead of
decimal digits.

Similarly, numbers starting with `0b` are interpreted using binary digits.

As before, change of base prefixes are case sensitive.

<!--wolf-->
```
-- These numbers are equal.
0xF_F
0b1111_1111
255
```

## Special numbers

A few names are reserved for useful special numbers:

- `nan` - the floating point "Not a Number", as from `0 / 0`
- `inf` - the largest positive floating point number, positive infinity
- `eul` - the closest number to Euler's number
- `tau` - the closest number to tau, the ratio of radius to circumference
- `phi` - the closest number to phi, the golden ratio