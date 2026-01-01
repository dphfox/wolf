---
layout: page
title: Lockfiles
page_number: 7
---

While Wolf requires full ahead-of-time knowledge of types and other constructs, it allows the programmer to omit many explicit declarations from their code for conciseness.
However, this can lead to situations where small code changes cascade in a massive, non-local manner.

## Worked example: type checking

Using type-checking as an example, consider this function which implicitly returns `int`.

<!--wolf-->
```
let cool_number = fn [] 5
```

This, in turn, can cause other function definitions to implicitly return `int`.

<!--wolf-->
```
let implicitly_int = fn [] cool_number [] * 2
```

Therefore, if the original function definition were to change its return type, it would also change the return type of an unknown number of other non-local functions.
In the worst case, this could make its way into a public-facing stable API and cause a breaking change!

## Explicit declarations

For this reason, many languages opt to force programmers to express _all_ information about their program explicitly in the source code.
In this case, that would mean the programmer must fully explicitly type any function signatures.
This ensures that a function's signature cannot change without a programmer explicitly altering its signature.

This has downsides of its own, though:
- Complex function signatures can become very unwieldy to write, especially code returning complex types.
- In some languages, the signature may need to represent information that cannot be annotated by the syntax.

In turn, these downsides may motivate efforts to ensure all compiler information can be annotated by a user of the language.
However, this can become increasingly hard, especially when dealing with ephemeral constructs invented by the compiler for its own internal static analysis.
This also ends up in tension with the goal of keeping the language human-writable with low mental overhead.

As such, Wolf looks for an alternative method of keeping code stable as it evolves.

## Lockfiles

To strike a balance between ergonomics and stabilisation, Wolf promotes the use of "lockfiles" for source code.
These are similar to the lockfiles used by package managers such as Cargo; Wolf treats the source file as a human interface for expressing the programmer's current intention, while using the lockfile to anchor that intention over time and ensure it is reproducible.

When a programmer writes Wolf code, they're free to only express as many constraints as necessary for the compiler to infer the types of the program.
Upon first compilation, the compiler commits this inferred knowledge to a `.lock` file next to the source file.
On subsequent compilations of that source file, the inferred knowledge is compared against the lockfile.
Any discrepancy causes a compile-time error, which can be resolved either by updating the lockfile, or by reverting the source changes.

This prevents cascading inference changes right away; in the prior example, even if `cool_number` updates its lockfile data to confirm its new return type, the lockfile data for `implicitly_int` still encodes its original return type.
As such, the compile-time error only moves one step downstream.
This allows for a human to properly audit the transitive effects of changes to exported API surfaces.

## Other uses

Beyond type checking, lockfiles can be used for other kinds of anchoring. 
For example, a networked app may choose to generate UUIDs automatically for the members of a replicated data structure. 
This could be stored in the lockfile to ensure that the UUIDs for members remains stable across code changes, without requiring the UUIDs to be manually authored and written inline in the source file.