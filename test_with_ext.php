<?php

include "test_header.php";

$testName = "PHP Ext";

$i = 0;

$checker = new SpellCheck("en");
var_dump($checker->check("wagon"));
var_dump($checker->check("house"));
var_dump($checker->check("fleur"));
var_dump($checker->check("boat"));
var_dump($checker->check("bateau"));

// while ($i < $testsLoop) {

//     $misspellings = $checker->check($content);
    
//     foreach ($misspellings as $misspelling) {
//         print_r([
//             //$misspelling->misspelled,
//             //$misspelling->pos,
//             $misspelling->line,
//             //$misspelling->suggestions
//         ]);
//     }

//     unset($misspelling);
//     $i++;
// }

include "test_footer.php";
