FROM rust:1.62

WORKDIR /usr/src/app
COPY . .

RUN ls -l
RUN cargo install --path .

# set the startup command to run your binary
# CMD ["/bin/ls", "-l"]
# TODO: add ability to inject redis host in Dockerfile (from docker-compose.yml)
CMD ["./target/release/rust-url-shortener", "-p", "9001", "-redishost", "st-redis"]
