FROM docker.io/ubuntu:latest

#Needed by diesel
RUN apt-get update && apt-get install -y libpq-dev

# Context of build must be root of this repository, not the directory containing this Dockerfile. See build.sh for hints.
COPY ./core/target/release/block_divider ./app/block_divider
COPY ./frontend/build /var/www/block_divider

RUN chmod +x ./app/block_divider

CMD ["./app/block_divider"]