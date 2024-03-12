<?php

include "test_header.php";

$testName = "PHP Ext";

$i = 0;

$checker = new SpellCheck("fr_FR");

while ($i < $testsLoop) {

    $misspellings = $checker->check($content);
    
    foreach ($misspellings as $misspelling) {
        print_r([
            //$misspelling->misspelled,
            //$misspelling->pos,
            $misspelling->line,
            //$misspelling->suggestions
        ]);
    }

    unset($misspelling);
    $i++;
}

include "test_footer.php";