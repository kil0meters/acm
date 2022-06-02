FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/acm
COPY . .

RUN cargo +nightly install --path server; \
    cargo +nightly install --path ramiel

WORKDIR /usr/src/acm/lilith
RUN wget https://github.com/WebAssembly/binaryen/releases/download/version_108/binaryen-version_108-x86_64-linux.tar.gz; \
    tar -xf binaryen-version_108-x86_64-linux.tar.gz; \
    cp binaryen-version_108/bin/wasm-opt /usr/bin/wasm-opt; \
    cargo install trunk; \
    rustup target add wasm32-unknown-unknown; \
    trunk build --release

FROM debian:bullseye-slim
RUN apt-get update; \
    apt-get install -y openssl libxml2 wget libxkbcommon-x11-0 libncurses5

RUN wget https://github.com/wasmerio/wasmer/releases/download/2.2.1/wasmer-linux-amd64.tar.gz; \
    tar -xf wasmer-linux-amd64.tar.gz --one-top-level; \
    cp wasmer-linux-amd64/bin/wasmer /usr/local/bin/wasmer; \
    rm -rf wasmer-linux-amd64 wasmer-linux-amd64.tar.gz

RUN wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-15/wasi-sdk_15.0_amd64.deb; \
    dpkg -i wasi-sdk_15.0_amd64.deb; \
    rm -f wasi-sdk_15.0_amd64.deb

RUN apt-get purge -y wget; \
    apt-get autoremove -y; \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
COPY --from=builder /usr/local/cargo/bin/ramiel /usr/local/bin/ramiel
COPY --from=builder /usr/src/acm/dist /usr/share/acm/dist
COPY scripts/start_acm.sh /usr/local/bin/start_acm

CMD ["start_acm"]
