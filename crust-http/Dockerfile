# Build stage
FROM rust:1.83 as Builder

RUN apt-get update && \
    apt-get install -y ca-certificates curl git && \
    update-ca-certificates

WORKDIR /usr/src/crust_http
COPY . .

# Build the release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install OpenSSL development libraries
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates curl git && \
    update-ca-certificates

# Copy only the compiled binary from the Builder stage
COPY --from=Builder /usr/src/crust/target/release/crust_http /app/crust_http

EXPOSE 3000

# Run the binary
CMD ["/app/crust-http"]
