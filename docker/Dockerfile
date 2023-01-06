# Stage: planner
FROM rust:1.66-buster AS planner

WORKDIR /app

RUN cargo install cargo-chef --locked
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage: builder
FROM rust:1.66-buster AS builder

WORKDIR /app

RUN cargo install cargo-chef --locked
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

COPY . .
RUN cargo build

# Stage: runner
FROM debian:buster AS runner

WORKDIR /app

COPY --from=builder /app/target ./target

ENTRYPOINT ["/app/target/debug/kb-api"]