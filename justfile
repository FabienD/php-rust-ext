# Simple extension php test
test_with_ext:
     php -d extension=./target/debug/libispell.so test_with_ext.php

# Simple Pure php test
test_pure_php:
     php test_pure_php.php

# Display PHP info
info:
     php -d extension=./target/debug/libispell.so info.php
