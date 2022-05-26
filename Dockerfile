# 1 use the Rust official image
FROM --platform=linux/amd64 rust:1.61-slim-buster

# 2 copy the files in the machine to the Docker image
COPY ./ ./

# 3 build the program for release
RUN cargo build --release

# 4 run the binary
CMD ["./target/release/rust-url-shortener"]

## instead of building the image on my laptop, try:
 # 1 build it in github actions and try to run on the misc server:
    # https://github.com/marketplace/actions/build-and-push-docker-images

 # 2 just try to build it in github actions and copy the exe to misc server
    # https://zellwk.com/blog/github-actions-deploy/
