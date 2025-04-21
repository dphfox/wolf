---
layout: page
title: Operators
page_number: 15
---

For convenience, Wolf allows arithmetic operators instead of functions. Unlike
regular calls, these operators have their own usage rules.

## Table of operations

This table summarises all of the symbol operators in Wolf.

In particular, this table shows how they relate to each other, via:

- **Priority** - Higher priority operators are evaluated before lower priority
operators in an expression.
- **Evaluates** - When many operators appear in a row, the order in which they
will be evaluated.



| Syntax     | Priority  | Evaluates     | <small>(like this)</small> | Description
|------------|-----------|---------------|----------------------------|------------------------
| `A.B`      | ▲ ▲ ▲ ▲ ▲ | Left to right | `(A.B).C`                  | Indexing into data
| `A B`      | ▲ ▲ ▲ ▲   | Right to left | `A (B (C))`                | Function evaluation
| `A...`     | ▲ ▲ ▲     | Single [^u]   | `(A)...`                   | Tuple flattening
| `A ^ B`    | ▲ ▲ ▲     | Right to left | `A ^ (B ^ C)`              | `exponent [A, B]`
| `-A`       | ▲ ▲       | Single [^u]   | `-(A)`                     | `negate A`
| `+A`       | ▲ ▲       | Single [^u]   | `+(A)`                     | `double_negate A`
| `A * B`    | ▲         | Left to right | `(A * B) * C`              | `multiply [A, B]`
| `A / B`    | ▲         | Left to right | `(A / B) / C`              | `divide [A, B]`
| `A // B`   | ▲         | Left to right | `(A // B) // C`            | `floor_divide [A, B]`
| `A % B`    | ▲         | Left to right | `(A % B) % C`              | `floor_mod [A, B]`
| `A + B`    |           | Left to right | `(A + B) + C`              | `add [A, B]`
| `A - B`    |           | Left to right | `(A - B) - C`              | `subtract [A, B]`
| `A == B`   | ▼         | Left to right | `(A == B) == C`            | `equals [A, B]`
| `A != B`   | ▼         | Left to right | `(A != B) != C`            | `not_equals [A, B]`
| `A < B`    | ▼         | Special [^i]  | `A < (B) < C`              | `less_than [A, B]`
| `A > B`    | ▼         | Special [^i]  | `A > (B) > C`              | `more_than [A, B]`
| `A <= B`   | ▼         | Special [^i]  | `A <= (B) <= C`            | `less_or_equals [A, B]`
| `A >= B`   | ▼         | Special [^i]  | `A >= (B) >= C`            | `more_or_equals [A, B]`
| `A & B`    | ▼ ▼       | Left to right | `(A & B) & C`              | `and [A, B]`
| `A | B`    | ▼ ▼ ▼     | Left to right | `(A | B) | C`              | `or [A, B]`
| `A -> B`   | ▼ ▼ ▼ ▼   | Left to right | `(A -> B) -> C`            | Manual chaining
| `A => B`   | ▼ ▼ ▼ ▼   | Left to right | `(A => B) => C`            | Automatic chaining

[^u]: Operators that only act on a single piece of data cannot appear multiple times in a row.
[^i]: Inequality operators can appear once (`x < size`) or twice (for example, `0 <= x < size`). These form single- and double-ended ranges respectively. No further inequality operators are allowed.