<?php

$text = "A tuple is a colleection of values of different types. 
Tuples are constructed using parentheses (), and each tuple itself is a value with type signature (T1, T2, ...), where T1, T2 are the types of its members. 
Functions can use tuples to return multiple values, as tuples can hold any number of values.
";

var_dump(ispell_check($text, "en"));
