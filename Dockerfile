# 1 build:
# docker build -t rus .
# 2 run:
# docker run -it --rm --name rus1 -e RUS_REDIS_HOST='0.0.0.0' rus

FROM rust:1.66 as builder

WORKDIR /rus
COPY . .

RUN cargo build --release

# our final base
FROM rust:1.66-slim-buster

# copy the build artifact from the build stage
COPY --from=builder /rus/target/release/rust-url-shortener .

# set the startup command to run your binary
CMD ["./rust-url-shortener", "-p", "9001"]
