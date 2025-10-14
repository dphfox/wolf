---
layout: page
title: Imperativeness
page_number: 2
---

Most modern programming languages bake imperative ideas into the fundamentals of the language design; for example, these languages have instruction pointers and mutable state.

Wolf very intentionally avoids this - not to avoid memory safety issues, but instead to try and work at a higher level of abstraction.

Wolf's perspective is that the imperative paradigm extends the rules of a more general "declarative" paradigm. 
The key extension is that imperative programs can depend on their history, with mutation being the mechanism for writing to that history.

The idea of mutation, and imperative logic more generally, is certainly useful in computing; the Turing machine uses inherently imperative logic to perform computation. 
As such, the intention of Wolf is not to eliminate imperative logic. 

However, Wolf does not see imperative as the "base" paradigm which others are built on top of. 
Instead, Wolf takes the view that declarative is the base paradigm, as with mathematics, and that the imperative programming is derived from the declarative paradigm. 

You can derive other paradigms too, like functional and reactive paradigms, which have more in common with declarative logic than they do with imperative logic. 
So, by attempting to explicitly model declarative logic as the base of the language, Wolf attempts to be far more paradigm-agnostic than many other programming languages, allowing you to use the right paradigm for the right part of your own programs, and let you port your core logic between all of them.