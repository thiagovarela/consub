FROM rust:1 AS chef 

RUN cargo install cargo-chef 

RUN apt update && apt install protobuf-compiler -y

WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 

COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

ENV SQLX_OFFLINE=true
RUN cargo build -p api --release

FROM gcr.io/distroless/cc:nonroot

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/release/api ./

EXPOSE 8000

CMD ["/app/api"]
