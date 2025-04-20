"./libfrt/std.frt" include

12 -> n
[
	n . "\n" .
	n 2 % 0 =
	if
		n 2 / -> n
	ifend
	! if
		n 3 * 1 + -> n
	ifend
	drop
	n 1 =
	if
		1 .
		exit
	ifend
	drop
]
