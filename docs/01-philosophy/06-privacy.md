---
layout: page
title: Privacy
page_number: 6
---

Many languages have an explicitly modelled notion of "public" and "private" access control.
However, Wolf explicitly chooses not to implement a feature like this.
Wolf prefers to base all access control _exclusively_ on lexical scope; anything which is in scope to you, is always fully accessible.

Suppose we have an object which we would like to divide into "public" and "private" data sections.

<!--wolf-->
```
let person = ty [
	name : str,
	age  : num,

	ultra_secret_password: str,
	ultra_sensitive_admin_rights: bool
]
```

This would be done by explicitly breaking up the definition into public and private "parts" that can be exposed separately.

<!--wolf-->
```
let person = ty [
	name : str,
	age  : num
]

let person_sensitive = ty [
	password     : str,
	admin_rights : bool
]
```

Since the types are now separate, they can be backed by two different data sources.
The visibility of the data sources determines who can access the data stored inside each.

<!--wolf-->
```
let people = ty [... person]
let people_sensitive = ty [... person_sensitive]
```

As a result, when describing a person with associated sensitive data, you would use multiple indices, one for each data source.
This has the additional benefit that code _not_ dealing with sensitive information about people would not bear the memory cost of the sensitive data section.
It also provides safety, in that other code can construct (or perhaps even register) people, but cannot register sensitive information about them.