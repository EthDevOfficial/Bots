FROM rust:latest as builder
WORKDIR /usr/src/target-backrunner-rust
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
# install additional dependencies after update, before rm -rf
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/target-backrunner-rust/abis/* /usr/local/abis/
COPY --from=builder /usr/local/cargo/bin/target-backrunner-rust /usr/local/bin/target-backrunner-rust
CMD ["target-backrunner-rust"]