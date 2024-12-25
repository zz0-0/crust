FROM rust:1.83 as builder

RUN apt-get update && \
    apt-get install -y ca-certificates curl git && \
    update-ca-certificates

WORKDIR /usr/src/crust

COPY . . 

RUN cargo build --release --workspace

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates dnsutils && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/crust/target/release/crust-test /app/crust-test

EXPOSE 3000
CMD ["/app/crust-test"]