---
layout: page
title: Objects
page_number: 1
---

Wolf intentionally does not implement traditional objects. That is to say, there
is no construct in Wolf that lets you index into a pointer using an identifier
in order to access a field or a method.

Instead, Wolf stores data and functions externally to objects, in typical
name-spaced and lexically scoped locations. Functions and data are free-standing
and are not "associated with" objects. This solves a variety of problems where
functions or data have to be mapped to a flat namespace of identifiers, e.g. the
"diamond problem". It also nicely solves the extensibility problem often found
with objects, without having to use a complex trait and impl system.

*Where languages like Rust define structs and enums, Wolf defines freestanding
structures and encodes the relationships they're allowed to have to each other.*

When data and functions are decoupled from objects in this way, the only remaining
property an object has is its _identity_, which is fundamentally irreducible.
Without identity, an object would simply be a pile of data.

That's why Wolf explicitly models objects as identities. An identity can be
passed into a function to refer to an object opaquely, and can be passed into a
named storage structure to retrieve information about it from memory. 


As a nice side effect, modelling identity explicitly means that it's really easy
to see where uncontrolled mutation or side effects may arise. Code without
identity is necesssarily operating on pure "piles of data" which cannot have
side effects, and so does not need to be checked for aliasing rules, ownership,
etc. The complexities introduced by OOP-style languages are opt-in by choosing
to use identities.