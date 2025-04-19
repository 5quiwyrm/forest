91 -> test
2 -> ctr

:: % -> __modulus -> __temp __temp __modulus / __modulus * __temp swap - ;

[
	ctr test =
	if
		test
		str
		"\sis\sprime.\n"
		<>
		.
		exit
	ifend
	drop
	test ctr %
	0 =
	if
		test str
		"\sis\snot\sprime,\s"
		<>
		"it\sis\sdivisible\sby\s"
		<>
		ctr str <>
		".\n" <>
		.
		exit
	ifend
	drop
	ctr 1 + -> ctr
]

