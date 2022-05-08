FROM rust:1.59 as builder
WORKDIR /usr/src/demo
COPY ./demo .
RUN cargo install --path .

FROM ubuntu:20.04

RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
RUN apt-get update && apt-get install postgresql -y
COPY --from=builder /usr/local/cargo/bin/demo /usr/local/bin/demo
