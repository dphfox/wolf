---
layout: page
title: Requests
page_number: 17
---

In theory, expressions should only operate on their input data. 
In practice this is difficult, because programs often require access to data sources and extra services for processing.

Wolf allows code to request and pass in extra dependencies without polluting the main path of computation.

## Basic use

Dependencies can be requested in any expression by writing:

- The `req` keyword.
- A capture or type name, describing the type of data that should be requested.

Any user of the expression in will implicitly also request the dependency.

<!--wolf-->
```
-- Request a number to be passed in.
let double_the_number = fn [] req num * 2

-- Implicitly also requests a number.
let quadruple_the_number = fn [] double_the_number [] * 2
```

## Providers

Values can be provided for requests inside of a block:

- The `prov` keyword
- The value to be provided.
- The block where the value will be served to requests.

<!--wolf-->
```
let double_the_number = fn [] req num * 2
let quadruple_the_number = fn [] double_the_number [] * 2

-- `4` will be sent to the `req num` in `double_the_number`.
let sixteen = prov 4 ( quadruple_the_number [] )
```

Wolf checks that all requests are matched with providers at compile time.
If a request isn't provided, the program won't compile.

<!--wolf-->
```
let double_the_number = fn [] req num * 2
let quadruple_the_number = fn [] double_the_number [] * 2

-- This is not OK - we haven't provided a number.
let sixteen = quadruple_the_number []
```

Providers shadow each other topologically; requests will use the most nested provider that satisfies the request.

<!--wolf-->
```
-- `req num` picks the most nested option, so it uses 10 instead of 2.
let forty = prov 2 (
	prov 10 (
		quadruple_the_number []
	)
)
```