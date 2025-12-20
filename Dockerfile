# Build stage
FROM rust:1.66 as builder

WORKDIR /rus
COPY . .

RUN cargo build --release

# Runtime stage
FROM rust:1.66-slim-buster

# Copy the build artifact from the build stage
COPY --from=builder /rus/target/release/rust-url-shortener /usr/local/bin/rust-url-shortener

# Create log directory
RUN mkdir -p /var/log/rust-url-shortener

# Expose port 1990
EXPOSE 1990

# Set the startup command to run your binary
# Port and other args can be overridden via CMD or docker run
CMD ["/usr/local/bin/rust-url-shortener", "-p", "1990"]
