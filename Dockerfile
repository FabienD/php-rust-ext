# PHP + RUST + Clang
FROM myprod/php:8.2-fpm as dev

LABEL maintainer="fabien@myprod.net"


# Install Clang, RUST
RUN apt-get update && apt-get install -y --no-install-recommends \
    clang \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y


# Build PHP extension
FROM dev as build

WORKDIR /var/www/rust

RUN cargo build --realease


# Install Build extension PHP extensions
FROM  myprod/php:8.2-fpm as runtime

COPY --from=build /var/www/rust/target/release/libext.so /etc/php/8.2/ext/ext.so

RUN echo "extension=ext.so" > /etc/php/8.2/mods-available/ext.ini \
    && ln -s /etc/php/8.2/mods-available/ext.ini /etc/php/8.2/fpm/conf.d/20-ext.ini \
    && ln -s /etc/php/8.2/mods-available/ext.ini /etc/php/8.2/cli/conf.d/20-ext.ini
