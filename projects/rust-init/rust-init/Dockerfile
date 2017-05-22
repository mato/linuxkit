FROM debian:stretch
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -qq -yy \
        --no-install-recommends \
        build-essential \
        ca-certificates \
        curl \
        patchelf \
    && apt-get clean
RUN curl -Ssf https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init -o /tmp/rustup-init \
    && chmod +x /tmp/rustup-init && /tmp/rustup-init -y
RUN PATH=$PATH:/root/.cargo/bin rustup target install x86_64-unknown-linux-musl
COPY . /build
WORKDIR /build
RUN PATH=$PATH:/root/.cargo/bin cargo build --target=x86_64-unknown-linux-musl
RUN patchelf --set-interpreter /lib/ld-musl-x86_64.so.1 target/x86_64-unknown-linux-musl/debug/init

FROM alpine:edge@sha256:8d428a798c87fb4c7fa7349a0a224da36e7c1ba2099610a164e346f220c18c49
ENTRYPOINT []
CMD []
WORKDIR /
COPY --from=0 /build/target/x86_64-unknown-linux-musl/debug/init /init
COPY etc etc/
