FROM rust:1.78.0 as builder

COPY ./ ./

RUN cargo build

FROM rust:1.78.0-slim

COPY src/static ./src/static
COPY --from=builder /target/debug/myapp ./target/debug/myapp
COPY --from=builder .env ./.env

CMD ["./target/debug/myapp"]