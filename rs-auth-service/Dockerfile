# BUILDER
FROM messense/rust-musl-cross:aarch64-musl as builder
RUN rustup target add aarch64-unknown-linux-musl		
RUN USER=root cargo new --bin rs-auth-service
WORKDIR ./rs-auth-service
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release 
RUN rm src/*.rs

ADD . ./
RUN rm ./target/aarch64-unknown-linux-musl/release/deps/rs_auth_service*
COPY templates ./templates
RUN cargo build --release 

#IMAGE 
FROM arm64v8/alpine:latest
ARG APP=/usr/src/rs-auth-service
EXPOSE 8000
ENV TZ=Etc/UTC \
  APP_USER=appuser

RUN addgroup -S $APP_USER \
  && adduser -S -g $APP_USER $APP_USER

RUN apk update \
  && apk add --no-cache ca-certificates tzdata\
  && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/rs-auth-service/target/aarch64-unknown-linux-musl/release/rs-auth-service ${APP}/rs-auth-service

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}
CMD ["./rs-auth-service"]