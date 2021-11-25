# GLOBAL VARS
ARG APP_NAME=rs_auth_service
#ARG PLATFORM=aarch64-unknown-linux-musl
ARG PLATFORM=x86_64-unknown-linux-musl 
# BUILDER
#FROM messense/rust-musl-cross:aarch64-musl as builder
FROM messense/rust-musl-cross:x86_64-musl as builder
ARG APP_NAME
ARG PLATFORM 
RUN rustup target add $PLATFORM
RUN USER=root cargo new --bin $APP_NAME
WORKDIR ./$APP_NAME
COPY Cargo.toml Cargo.lock ./
COPY templates ./templates
RUN cargo build --release
RUN rm src/*.rs

ADD . ./
RUN rm ./target/$PLATFORM/release/deps/$APP_NAME*
RUN cargo build --release 

#IMAGE 
FROM alpine:latest
ARG APP_NAME
ARG PLATFORM
ARG APP_DIR=/usr/src/$APP_NAME
EXPOSE 8000
ENV TZ=Etc/UTC \
  APP_USER=appuser

RUN addgroup -S $APP_USER \
  && adduser -S -g $APP_USER $APP_USER

RUN apk update \
  && apk add --no-cache ca-certificates tzdata\
  && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/$APP_NAME/target/$PLATFORM/release/$APP_NAME ${APP_DIR}/$APP_NAME

RUN chown -R $APP_USER:$APP_USER ${APP_DIR}

USER $APP_USER
WORKDIR ${APP_DIR}
CMD ["./rs_auth_service"]