# PHP + RUST + Clang
FROM myprod/php:8.3-fpm

LABEL maintainer="fabien@myprod.net"

# Prepare devcontainer
RUN apt-get update && apt-get install -y --no-install-recommends \
    clang \
    aspell \
    aspell-fr \
    aspell-en \
    && mkdir -p /workspaces

# Switch user
USER dockeruser

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
