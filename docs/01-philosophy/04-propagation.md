---
layout: page
title: Propagation
page_number: 4
---

Wolf generally expects and encourages people to write programs forward, starting at inputs and gradually deriving intermediate steps until arriving at an output.
It's the most natural way to program, mimicking the process of writing prose; later ideas build on earlier ones.

In recognition of this, Wolf syntax is designed to propagate information preferentially forward.
Local inference flows down topologically through a program from start to end.
Syntax is generally placed with constraints first and values later, and suffix operations are avoided.

This style of syntax ensures that, as you write out a program, you get useful autocomplete behaviour as you go. 
It also ensures as you add code to your program, the meaning of previously written lines don't change.

This doesn't mean all Wolf code has to be ordered top-to-bottom; Wolf explicitly avoids that.
Sometimes unordered representation is better for things that aren't topologically related.
However, when you're expressing a chain of reasoning, forwards propagation is more intuitive.