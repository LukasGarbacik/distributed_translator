FROM rust:1.70-slim as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /usr/src/app/target/release/rust_project /usr/local/bin/rust_project

CMD ["rust_project"]