:: println . "\n" . ;

1 -> a
1 -> b
0 -> i
5 => limit

:: fib
	b a - -> a
	a println
	b a + -> b
	i 1 + -> i
	i limit = !
	if
		drop fib
	ifend
;
fib

exit