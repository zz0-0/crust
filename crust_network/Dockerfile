FROM rust:1.83 AS builder

RUN apt-get update && \
    apt-get install -y ca-certificates curl git && \
    update-ca-certificates

WORKDIR /usr/src/app/

COPY crust_core/Cargo.toml ./crust_core/Cargo.toml
COPY crust_core/src ./crust_core/src

COPY crust_network/Cargo.toml ./crust_network/Cargo.toml
COPY crust_network/src ./crust_network/src

RUN cd crust_network && cargo build --release --target-dir /usr/src/app/target

FROM debian:bookworm-slim

RUN apt-get update && \ 
    apt-get install -y libssl-dev ca-certificates dnsutils && \ 
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/crust_network /usr/local/bin/crust_network
RUN chmod +x /usr/local/bin/crust_network

EXPOSE 8000

CMD ["/usr/local/bin/crust_network"]