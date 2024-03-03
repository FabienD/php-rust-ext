# Run cargo build debug
build:
     cargo build --release

# Simple extension php test
ext_php:
     php -d extension=./target/release/libispell.so test_with_ext.php

# Simple Pure php test
pure_php:
     php test_pure_php.php

# Display PHP info
info:
     php -d extension=./target/release/libispell.so info.php
