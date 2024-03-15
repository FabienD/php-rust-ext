<?php

declare(strict_types=1);

$content = file_get_contents(__DIR__ . '/features/text_fr.txt');
$testsLoop = 100;

memory_reset_peak_usage();

$start = microtime(true);
