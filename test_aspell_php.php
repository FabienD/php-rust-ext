<?php

include "test_header.php";

use PhpSpellcheck\Spellchecker\Aspell;

require_once __DIR__ . '/vendor/autoload.php';

$testName = "PHP Ext";
$checker = Aspell::create();

$misspelling_counter = 0;

while ($i < $testsLoop) {    
    $misspellings = $checker->check($content, ['fr_FR']);
        
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
