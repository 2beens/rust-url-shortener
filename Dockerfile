FROM rust:1.62 as build

# create a new empty shell project
RUN USER=root cargo new --bin urlshortener
WORKDIR /urlshortener

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN echo "-----> contents"
RUN echo $(pwd)
RUN echo $(ls -l)
RUN rm ./target/release/deps/rust-url-shortener*
RUN cargo build --release

# our final base
FROM rust:1.62

# copy the build artifact from the build stage
COPY --from=build /urlshortener/target/release/rust-url-shortener .

# set the startup command to run your binary
CMD ["./rust-url-shortener"]
