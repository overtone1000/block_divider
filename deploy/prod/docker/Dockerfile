FROM ubuntu:24

COPY ../../../core/target/release/block_divider bin/block_divider/block_divider
COPY ../../../frontend/build /var/www/block_divider

ENTRYPOINT ["block_divider"]