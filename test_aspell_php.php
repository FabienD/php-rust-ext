<?php

include "test_header.php";

use PhpSpellcheck\Spellchecker\Aspell;

require_once __DIR__ . '/vendor/autoload.php';

$testName = "Tigitz PHP (Aspell bin execution)";
$checker = Aspell::create();

foreach($tests as $locale => $featureFile) {
    $content = file_get_contents(__DIR__ . $featureFile);
    $misspelling_counter[$locale] = 0;
    $i = 0;
    
    while ($i < $testsLoop) {    
        $misspellings = $checker->check($content, [$locale]);
            
        foreach ($misspellings as $misspelling) {
            $misspelling_counter[$locale]++;
            // print_r([
            //     $misspelling->getWord(),
            //     $misspelling->getLineNumber(),
            //     $misspelling->getOffset(),
            //     $misspelling->getSuggestions(),
            // ]);
        }
        $i++;
    }
}

include "test_footer.php";
