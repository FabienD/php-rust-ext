<?php

$total = microtime(true) - $start;
$memory = memory_get_usage() / 1024 / 1024;
$peek = memory_get_peak_usage() / 1024 / 1024;

echo "
$testName execution time in seconds : $total

Misspelling count : $misspelling_counter

Memory usage : $memory Mo
Memory peek usage : $peek Mo
";
