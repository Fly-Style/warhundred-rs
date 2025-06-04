FROM rust:1.87.0 AS chef

RUN apt-get update \
    && apt-get install -y --no-install-recommends build-essential ca-certificates cmake gcc libc6-dev pkg-config libprotobuf-dev protobuf-compiler tree wget \
    && rm -rf /var/lib/apt/lists/*

# We only pay the installation cost once,
# it will be cached from the second build onwards
RUN cargo install cargo-chef
WORKDIR app


FROM chef AS planner
# Copy the whole project
COPY . .
# Prepare a build plan ("recipe")
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

ARG COMPILE_PROFILE
ENV COMPILE_PROFILE=${COMPILE_PROFILE}

# Copy the build plan from the previous Docker stage
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this layer is cached as long as `recipe.json`
# doesn't change.
RUN cargo chef cook $COMPILE_PROFILE --recipe-path recipe.json

# Build the whole project
COPY . .
RUN cargo build $COMPILE_PROFILE --bin main

FROM alpine:3.22

WORKDIR /app

ARG TARGET_PROFILE
ENV TARGET_PROFILE=${TARGET_PROFILE}

COPY --from=builder /app/target/${TARGET_PROFILE}/main /bin