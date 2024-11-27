---
layout: page
title: Expressions
---

Wolf provides a few built-in expression notations.

Operations are left associative, except for exponentiation.

| Operation          | Written as | Precedence | Notes |
|--------------------|------------|------------|-------|
| Or                 | `A | B`    | 1
| And                | `A & B`    | 2
| Less than          | `A < B`    | 3
| More than          | `A > B`    | 3
| Less or equal      | `A <= B`   | 3
| More or equal      | `A >= B`   | 3
| Equal              | `A == B`   | 3
| Not equal          | `A != B`   | 3
| Add                | `A + B`    | 4
| Subtract           | `A - B`    | 4
| Multiply           | `A * B`    | 5
| Divide             | `A / B`    | 5 | `/ 0` is ±∞, `0/0` is `nan`
| Floor divide       | `A // B`   | 5 | Rounds to -inf
| Modulo             | `A % B`    | 5 | Floored modulo
| Not                | `!A`       | 6
| Negate             | `-A`       | 6
| Double negate      | `+A`       | 6
| Count              | `#`        | 6
| Exponent           | `A ^ B`    | 7

Not all types implement all operations. For example, `num` does not implement
the boolean "or", "and", or "not" operations.

## Running an expression

Wolf supports evaluating expressions directly from the command line with the
`wf --` command. The result is printed to the output.

```
$ wf -- 2 + 2
4
```