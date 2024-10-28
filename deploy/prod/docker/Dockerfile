FROM ubuntu:24.04

# Context of build must be root of this repository, not the directory containing this Dockerfile. See build.sh for hints.
COPY ./core/target/release/block_divider /bin/block_divider/block_divider
COPY ./frontend/build /var/www/block_divider

ENTRYPOINT ["/bin/block_divider/block_divider"]