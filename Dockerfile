FROM rust:1.62

WORKDIR /usr/src/app
COPY . .

RUN ls -l
RUN cargo install --path .

# set the startup command to run your binary
# CMD ["/bin/ls", "-l"]
CMD ["./target/release/rust-url-shortener"]
