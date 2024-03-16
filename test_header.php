<?php

declare(strict_types=1);

$tests = [
    'en_US' => '/features/simple_en.txt',
    'fr_FR' => '/features/simple_fr.txt',
];

$testsLoop = 100;

print_r('----------------------------------------------------------');

memory_reset_peak_usage();

$misspelling_counter = [];

$start = microtime(true);
