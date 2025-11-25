---
layout: page
title: Blocks
page_number: 13
---

Blocks are used to scope calculations and definitions to a particular part of the program.

## Explicit blocks

As seen previously, parentheses `()` are used to define explicit start and end points for a block.

<!--wolf-->
```
let ten = (
	let four = 2 + 2
	four * four - 6
)
```

## Implicit blocks

In some cases, explicit blocks can lead to large amounts of nesting, especially when guarding against some conditions.

<!--wolf-->
```
fn perform_transaction [.account_id : num, .item_price : num] (
	let account = get_account [.id account_id]
	if !account.exists then throw error [.reason "Account does not exist"]
	if account.balance < item_price then throw error [.reason "Insufficient funds"] 
	else (
		let payment_method = account => get_payment_method
		if !payment_method.exists then throw error [.reason "Account has no payment method"]
		else (
			account => deduct [
				.from payment_method.primary
				.amount item_price
			]
		)
	)
)
```

To alleviate this, blocks can be implicitly scoped using a colon `:` instead of parentheses.
Implicit blocks run until the end of the first well-defined ancestor block.

<!--wolf-->
```
fn perform_transaction [.account_id : num, .item_price : num] (
	let account = get_account [.id account_id]
	if !account.exists then error [.reason "Account does not exist"]
	if account.balance < item_price then error [.reason "Insufficient funds"]
	else:

	let payment_method = account => get_payment_method
	if !payment_method.exists then error [.reason "Account has no payment method"]
	else:
	
	account => deduct [
		.from payment_method.primary
		.amount item_price
	]
)
```

Because implicit blocks run to the end, they aren't usable if syntax is required after the end of the block.

<!--wolf-->
```
-- This is not allowed - the `else` is interpreted as part of `then`'s block, which is not valid.
if !payment_method.exists
then: throw error [.reason "Account has no payment method"] 
else: account => deduct [
	.from payment_method.primary
	.amount item_price
]
```