# FROM rust:1.85 AS builder
# WORKDIR /usr/src/anthill
# COPY . .
# RUN cargo install --path .

# FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
# RUN apt-get update && apt-get install -y \
#     python3.13 \
#     python3-pip
# COPY --from=builder /usr/local/cargo/bin/anthill /usr/local/bin/anthill
# CMD ["anthill"]


FROM rust:1.85

RUN apt-get update && apt-get install -y \
    python3 python3-pip \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add clippy rustfmt
RUN pip install --break-system-packages pre-commit

ENV CARGO_HOME=/usr/local/cargo
ENV PATH="$CARGO_HOME/bin:${PATH}"

WORKDIR /workspaces/anthill