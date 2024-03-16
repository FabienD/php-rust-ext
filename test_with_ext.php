<?php

include "test_header.php";

$testName = "PHP Pure Rust Extension";

foreach($tests as $locale => $featureFile) {
    $content = file_get_contents(__DIR__ . $featureFile);
    $checker = new SpellCheck($locale);
    $misspelling_counter[$locale] = 0;
    $i = 0;
    
    while ($i < $testsLoop) {    
        $misspellings = $checker->check($content);
        
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
