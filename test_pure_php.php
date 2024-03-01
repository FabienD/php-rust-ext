<?php

declare(strict_types=1);

use PhpSpellcheck\Spellchecker\Aspell;

require_once __DIR__ . '/vendor/autoload.php';


$text = "A tuple is a colleection of values of different types. 
Tuples are constructed using parentheses (), and each tuple itself is a value with type signature (T1, T2, ...), where T1, T2 are the types of its members. 
Functions can use tuples to return multiple values, as tuples can hold any number of values.
";

$aspell = Aspell::create();
$misspellings = $aspell->check($text, ['en_EN']);

foreach ($misspellings as $misspelling) {
    print_r([
        $misspelling->getWord(),
        $misspelling->getLineNumber(),
        $misspelling->getOffset(),
    ]);
}
