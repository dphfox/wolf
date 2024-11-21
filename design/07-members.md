# Members

Wolf scripts contain various top-level *members*. Members are the foundational
building blocks of Wolf, like functions and type definitions.

The order of members doesn't matter.

### Identifiers

Every member has a unique name known as an *identifier*.

Identifiers are made of any combination of ASCII letters, numbers and
underscores, with the following limitations:

- Reserved keywords can't be used in identifiers.
- Identifiers that look like literal numbers are disallowed.
- Identifiers must have one non-underscore character (`_` is not valid).

Here are some examples of identifiers:

```
person
PEOPLE
Crowd
large_gathering
_smallGroup
3rdPerson
```