---
layout: page
title: Operations
page_number: 6
---

Operations are used to process and transform data in Wolf.

## Applying a function

Wolf allows you to apply functions to a single tuple of data at a time. 
This is done by prefixing the tuple with the function name.

<!--wolf-->
```
-- Negates 2.
negate [2]
```

The tuple can contain multiple data.

<!--wolf-->
```
-- Adds 9 and 10.
add [9, 10]
```

## Blocks

Blocks are declared with parentheses `()`.

They can be included in other expressions; the contents of the block are evaluated independently of the other expression, meaning they can change the order in which operations are done.

<!--wolf-->
```
-- The block evaluates to 8, then is multiplied by 2.
2 * (5 + 3)
```

Other kinds of block will be introduced later.

## Special notation

Certain functions in Wolf are important enough to have special notation.

<!--wolf-->
```
-- Standard arithmetic (including exponentiation).
2/5 + 4*3 - 6^4

-- Negation and double negation.
-2 * +6

-- Floor division and floor modulo.
4 // 3 % 5

-- Ceiling division.
4 /^ 3

-- Equality comparisons.
1 + 1 == 4 - 2 == 2

-- Equality comparisons and boolean combination.
9 == 9 and 10 == 10 or 19 != 21

-- Single-ended ranges.
(2 < 5) and (3 <= 3) or (4 > 5) and (6 >= 0)

-- Double-ended ranges (combines single-ended ranges).
(0 <= 5 < 21) or (20 >= 5 > -1)
```

The table below shows these *operators* and how they relate to each other. 
Higher priority operators are evaluated before lower priority operators in an expression.

Other non-operators are also included to complete the comparison.

---

| Syntax     | Function                     | Priority  | Notes
|------------|------------------------------|-----------|-------------
| `A [B]`    | *(function evaluation)*      | ▲ ▲ ▲ ▲ ▲     |
| `A.B`      | *(accessing named data)*     | ▲ ▲ ▲ ▲       |
| `!A`       | `invert A`                   | ▲ ▲ ▲         | Pick one[^unary]
| `-A`       | `negate A`                   | ▲ ▲ ▲         | Pick one[^unary]
| `+A`       | `double_negate A`            | ▲ ▲ ▲         | Pick one[^unary]
| `#A`       | `count A`                    | ▲ ▲ ▲         | Pick one[^unary]
| `A ^ B`    | `exponent [A, B, ...]`       | ▲ ▲           |
| `A * B`    | `multiply [A, B, ...]`       | ▲             |
| `A / B`    | `divide [A, B, ...]`         | ▲             |
| `A // B`   | `floor_divide [A, B, ...]`   | ▲             |
| `A /^ B`   | `ceil_divide [A, B, ...]`    | ▲             |
| `A % B`    | `floor_mod [A, B, ...]`      | ▲             |
| `A + B`    | `add [A, B, ...]`            |               |
| `A - B`    | `subtract [A, B, ...]`       |               |
| `A < B`    | `less_than [A, B]`           | ▼             | Must form an order[^ineq]
| `A > B`    | `more_than [A, B]`           | ▼             | Must form an order[^ineq]
| `A <= B`   | `less_or_equals [A, B]`      | ▼             | Must form an order[^ineq]
| `A >= B`   | `more_or_equals [A, B]`      | ▼             | Must form an order[^ineq]
| `A == B`   | `equals [A, B, ...]`         | ▼ ▼           |
| `A != B`   | `invert_equals [A, B, ...]`  | ▼ ▼           |
| `A and B`  | `and_values [A, B, ...]`     | ▼ ▼ ▼         |
| `A or B`   | `or_values [A, B, ...]`      | ▼ ▼ ▼ ▼       |
| `A -> B`   | *(manual chaining)*          | ▼ ▼ ▼ ▼ ▼     |
| `A => B`   | *(automatic chaining)*       | ▼ ▼ ▼ ▼ ▼     |

[^unary]: Unary operators cannot be mixed.

[^ineq]: `<` / `<=` cannot be mixed with `>` / `>=` at the same level.

## Integer conversion

Wolf does not implicitly convert between `num` and `int`. 
Instead, the arithmetic operators accept any mix of `int` or `num`, and return `int` if the types guarantee the result is an integer.

That means:

- `/` always returns `num`, because not all integers divide evenly.
- `//` and `/^` always returns `int`, because the result is floored.
- All other operators return `int` if all operands are `int`.

You can use this behaviour to explicitly convert between `num` and `int`:

<!--wolf-->
```
-- Converts the integer to the nearest floating-point number.
2 / 1
-- Floors the floating-point number towards the largest integer less than it.
2.4 // 1
-- Floors the floating-point number towards the smallest integer greater than it.
2.4 /^ 1
```

Note that `num` and `int` may handle overflow, underflow, and division by zero differently to each other.