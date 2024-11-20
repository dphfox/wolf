# The Wolf Scripting Language

Wolf is an experimental design for a static scripting language that transpiles
to Luau.

The goal of Wolf is to re-imagine Lua's "small but mighty" design as a static
language, rather than one revolving around dynamic features. How can a minimal,
easy-to-learn set of primitives compose into an expressive, high-level language?

## Goals

- *Approachable and thin.* Wolf is inspired by Luau, and transpiles close to
hand-written Luau, with a familiar developer experience.
- *Silently strongly typed.* Wolf infers a good definite type for every value in
the program, without forcing you to express each one.
- *Helpful resource management.* Wolf can track how resources are used across
your program with modern techniques, to enforce safety if you need it.
- *Import-lightness.* Wolf introduces new namespace-centric organisation that
transcends the filesystem, allowing for freeform, concise library usage.