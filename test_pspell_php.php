<?php

include "test_header.php";

use PhpSpellcheck\Spellchecker\PHPPspell;

require_once __DIR__ . '/vendor/autoload.php';

$testName = "PHP Ext";
$checker = new PHPPspell();

while ($i < $testsLoop) {    
    $misspellings = $checker->check($content, ['fr_FR'], ['Pspell context']);

    foreach ($misspellings as $misspelling) {
        $misspelling_counter++;
        // print_r([
        //     $misspelling->getWord(),
        //     $misspelling->getLineNumber(),
        //     $misspelling->getOffset(),
        //     $misspelling->getSuggestions(),
        // ]);
    }
    $i++;
}

include "test_footer.php";
