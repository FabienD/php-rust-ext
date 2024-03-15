<?php

declare(strict_types=1);

$content = file_get_contents(__DIR__ . '/features/simple_fr.txt');
$testsLoop = 1000;

memory_reset_peak_usage();

$i = 0;
$misspelling_counter = 0;

$start = microtime(true);
