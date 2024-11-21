# Operators

Wolf provides a few built-in operator notations.

Operations are left associative, except for exponentiation.

| Operation          | Written as | Precedence | Notes |
|--------------------|------------|------------|-------|
| Or                 | `A or B`   | 1
| Xor                | `A xor B`  | 2
| And                | `A and B`  | 3
| Less than          | `A < B`    | 4
| More than          | `A > B`    | 4
| Less or equal      | `A <= B`   | 4
| More or equal      | `A >= B`   | 4
| Equal              | `A == B`   | 4
| Not equal          | `A != B`   | 4
| Add                | `A + B`    | 5
| Subtract           | `A - B`    | 5
| Multiply           | `A * B`    | 6
| Divide             | `A / B`    | 6 | `/ 0` is ±∞, `0/0` is `nan`
| Floor divide       | `A // B`   | 6 | Rounds to -inf
| Modulo             | `A % B`    | 6 | Floored modulo
| Not                | `not A`    | 7
| Negate             | `-A`       | 7
| Double negate      | `+A`       | 7
| Exponent           | `A ^ B`    | 8

Parentheses can be used to force a desired order of operations.

```
2*1 + 5 + 3
2*(1 + 5) + 3
```

## Running an expression

Wolf supports evaluating expressions directly from the command line with the
`wf =` command. The result is printed to the output.

```
$ wf = 2 + 2
4
```