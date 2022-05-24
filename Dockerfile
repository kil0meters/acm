FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/acm
COPY . .

RUN cargo +nightly install --path server; \
    cargo +nightly install --path ramiel

WORKDIR /usr/src/acm/app
RUN cargo install trunk; \
    rustup target add wasm32-unknown-unknown; \
    trunk build --release

FROM debian:buster-slim
RUN apt-get update; \
    apt-get install -y openssl; \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
COPY --from=builder /usr/local/cargo/bin/ramiel /usr/local/bin/ramiel
COPY --from=builder /usr/src/acm/dist /usr/share/acm/dist
COPY scripts/start_acm.sh /usr/local/bin/start_acm

CMD ["start_acm"]
