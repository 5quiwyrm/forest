:: % -> __modulus -> __temp
	__temp __modulus / __modulus *
	__temp swap -
;
:: >= -> b -> a
	a b >
	a b =
	|
;
:: <= -> b -> a
	a b <
	a b =
	|
;
:: ^ -> b -> a
	a b ! &
	a ! b &
	|
;
