FROM rust:1.66

WORKDIR /usr/src/app
COPY . .

RUN ls -l
RUN cargo install --path .

# set the startup command to run your binary
CMD ["./target/release/rust-url-shortener", "-p", "9001", "-redishost", "127.0.0.1"]

# 1 build:
# docker build -t rus .
# 2 run:
# docker run -it --rm --name rus1 rus
