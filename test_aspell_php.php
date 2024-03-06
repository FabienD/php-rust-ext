<?php

include "test_header.php";

use PhpSpellcheck\Spellchecker\Aspell;

require_once __DIR__ . '/vendor/autoload.php';

$testName = "PHP Ext";
$i = 0;
$checker = Aspell::create();

while ($i < $testsLoop) {    
    $misspellings = $checker->check($content, ['fr_FR']);

    // foreach ($misspellings as $misspelling) {
    //     print_r([
    //         $misspelling->getWord(),
    //         $misspelling->getLineNumber(),
    //         $misspelling->getOffset(),
    //         $misspelling->getSuggestions(),
    //     ]);
    // }

    unset($misspelling);
    $i++;
}

include "test_footer.php";
