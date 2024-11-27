---
layout: page
title: Namespaces
page_number: 9
---

Within a workspace, Wolf does not impose boundaries between scripts.

Scripts can discover members from any other scripts by default, through any
combination of visiting siblings, going up a nesting level, or going down a
nesting level.

Instead, Wolf uses *namespaces* to separate content. When a script is given a
namespace, it becomes opaque to the outside world; members in the script, as
well as any nested scripts, can't be visited by discovery. This also works the
other way; the script, and any nested scripts, can't discover outside members.

Wolf encourages the use of namespaces coarsely - roughly one per package - but
sub-namespaces can be freely added where the encapsulation is useful.

## Syntax

To give a script a namespace, add the following to the head of the file:

```
namespace MyNamespace
```

Namespaces must be valid identifiers.