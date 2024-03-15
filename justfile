# Run cargo build debug
build:
     cargo build --release

# Simple extension php calling Aspell bin
ext_php:
     php -d extension=./target/release/libspellckeck.so test_with_ext.php

# Simple Pure php test calling Aspell bin
aspell_bin_php:
     php test_aspell_php.php

# Simple php test with Pspell ext
pspell_php:
     php test_pspell_php.php

# Display PHP info
info:
     php -d extension=./target/release/libspellckeck.so info.php
