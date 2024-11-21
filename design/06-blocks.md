# Blocks

Blocks are expressions that evaluate a series of sub-expressions in order.

Blocks are defined with the `do` keyword, and span until a matching `end`
keyword is found.

```
fn do_something = do 2 + 2 end
```

Multiple expressions can be run by separating them with semicolons `;`. The last
expression becomes the returned value.

```
fn do_something = do 2 + 2; 4 + 4; 6 + 6 end
```

Blocks can span multiple lines; if they do, then semicolons may be automatically
inserted by Wolf before lines that could start new expressions.

```
fn do_something = do
	2 + 2
	4 + 4
	6 + 6
end
```

Because blocks are commonly associated with functions, the `=` can be omitted.
This will apply to other structures down the line too.

```
fn do_something do
	2 + 2
	4 + 4
	6 + 6
end
```