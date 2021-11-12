# BUILDER
FROM messense/rust-musl-cross:aarch64-musl as builder
RUN rustup target add aarch64-unknown-linux-musl		
RUN USER=root cargo new --bin rs-api-service
WORKDIR ./rs-api-service
COPY Cargo.toml Cargo.lock ./
COPY templates ./templates
RUN cargo build --release 
RUN rm src/*.rs

ADD . ./
RUN rm ./target/aarch64-unknown-linux-musl/release/deps/rs_api_service*
RUN cargo build --release 

#IMAGE 
FROM arm64v8/alpine:latest
ARG APP=/usr/src/rs-api-service
EXPOSE 8000
ENV TZ=Etc/UTC \
  APP_USER=appuser

RUN addgroup -S $APP_USER \
  && adduser -S -g $APP_USER $APP_USER

RUN apk update \
  && apk add --no-cache ca-certificates tzdata\
  && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/rs-api-service/target/aarch64-unknown-linux-musl/release/rs-api-service ${APP}/rs-api-service

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}
CMD ["./rs-api-service"]