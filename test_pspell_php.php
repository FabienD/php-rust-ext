<?php

include "test_header.php";

use PhpSpellcheck\Spellchecker\PHPPspell;

require_once __DIR__ . '/vendor/autoload.php';

$testName = "PHP Ext";
$i = 0;
$checker = new PHPPspell();

while ($i < $testsLoop) {    
    $misspellings = $checker->check($content, ['fr_FR'], ['Pspell context']);

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
