FROM rust:slim as builder
run rustc --version
WORKDIR /app
COPY . .
RUN cargo build --release
RUN strip target/release/calendar

FROM debian:stable-slim
WORKDIR /app

COPY --from=builder /app/target/release/calendar .
RUN chmod u+x calendar

ENTRYPOINT ["./calendar"]
