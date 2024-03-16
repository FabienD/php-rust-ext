# PHP + RUST + Clang
FROM myprod/php:8.3-fpm

LABEL maintainer="fabien@myprod.net"

# Prepare devcontainer
RUN apt-get update && apt-get install -y --no-install-recommends \
    clang \
    aspell libaspell-dev libpspell-dev \
    aspell-fr aspell-en aspell-es \
    && mkdir -p /workspaces

RUN docker-php-ext-install pspell

# Switch user
USER dockeruser

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
