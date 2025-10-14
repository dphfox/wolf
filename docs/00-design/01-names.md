---
layout: page
title: Names
page_number: 1
---

Names are a string of characters written into a source file to refer to
something that's available to the program.

## Basic use

Names are enclosed in backticks. Any characters between the backticks are
interpreted as a name (except for backticks, which close the name).

<!--wolf-->
```
-- These are valid names.
`Hello, world!`
`12345`
`https://wolf.phfox.net/`
```

Conventionally, names are written in `snake_case`.

## Without backticks

Some simple names do not need backticks. 
Any contiguous span of letters and digits is interpreted as a name.

<!--wolf-->
```
-- These are valid names.
123          -- becomes: `123`
Hello        -- becomes: `Hello`
Testing12    -- becomes: `Testing12`
3Dimensional -- becomes: `3Dimensional`
```

In addition, a few extra characters are permitted without backticks:
- Underscores `_` are allowed anywhere in a name.
- One decimal point `.` is allowed per name; it must be surrounded by digits.

Any extra characters that don't follow those rules will not be included in the name unless backticks are used.

<!--wolf-->
```
-- These are valid names.
Hello_World    -- becomes: `Hello_World`
_Hello_World   -- becomes: `_Hello_World`
Testing_123    -- becomes: `Testing_123`
1_337          -- becomes: `1_337`
1__3.37        -- becomes: `1__3.37`
6.28           -- becomes: `6.28`
_              -- becomes: `_`

-- These are *not* valid names.
wolf.phfox.net -- becomes: `wolf` . `phfox` . `net`
127.0.0.1      -- becomes: `127.0` . `0.1`
.5             -- becomes: . `5`
100.foo        -- becomes: `100` . `foo`
```