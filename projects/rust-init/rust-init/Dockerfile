# Use sha256 here to get a fixed version
FROM alpine:edge@sha256:8d428a798c87fb4c7fa7349a0a224da36e7c1ba2099610a164e346f220c18c49
RUN apk add --update --no-cache rust
COPY ./src /src
WORKDIR /src
RUN rustc init.rs

FROM alpine:edge@sha256:8d428a798c87fb4c7fa7349a0a224da36e7c1ba2099610a164e346f220c18c49
ENTRYPOINT []
CMD []
WORKDIR /
COPY --from=0 /src/init /init
# XXX
COPY --from=0 /usr/lib/libunwind* /lib/
