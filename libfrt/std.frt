:: % -> __modulus -> __temp
	__temp __modulus / __modulus *
	__temp swap -
;
:: >= -> __b -> __a
	__a __b >
	__a __b =
	|
;
:: <= -> __b -> __a
	__a __b <
	__a __b =
	|
;
:: ^ -> __b -> __a
	__a __b ! &
	__a ! __b &
	|
;
:: println . "\n" . ;
