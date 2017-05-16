FROM debian:stretch
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -qq -yy \
        --no-install-recommends \
        build-essential \
        ca-certificates \
        patchelf \
    && apt-get clean
ADD https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init /tmp/
RUN chmod +x /tmp/rustup-init && /tmp/rustup-init -y
COPY ./src /src
WORKDIR /src
RUN PATH=$PATH:/root/.cargo/bin rustup target install x86_64-unknown-linux-musl
RUN PATH=$PATH:/root/.cargo/bin rustc --target=x86_64-unknown-linux-musl init.rs
RUN patchelf --set-interpreter /lib/ld-musl-x86_64.so.1 ./init

FROM alpine:edge@sha256:8d428a798c87fb4c7fa7349a0a224da36e7c1ba2099610a164e346f220c18c49
ENTRYPOINT []
CMD []
WORKDIR /
COPY --from=0 /src/init /init
