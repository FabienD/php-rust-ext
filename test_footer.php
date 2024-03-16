<?php

$total = microtime(true) - $start;
$memory = memory_get_usage() / 1024 / 1024;
$peek = memory_get_peak_usage() / 1024 / 1024;

print_r("
$testName execution time in seconds : $total

Misspelling count FR : " . $misspelling_counter["fr_FR"] . "
Misspelling count EN : " . $misspelling_counter["en_US"] . "

Memory usage : $memory Mo
Memory peek usage : $peek Mo
");
