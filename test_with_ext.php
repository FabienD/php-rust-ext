<?php

include "test_header.php";

$testName = "PHP Rust Ext";
$checker = new SpellCheck("fr_FR");

while ($i < $testsLoop) {

    $misspellings = $checker->check($content);
    
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
