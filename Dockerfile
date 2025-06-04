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

FROM debian:stable-slim

WORKDIR /app

ARG TARGET_PROFILE
ENV TARGET_PROFILE=${TARGET_PROFILE}

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libsqlite3-dev sqlite3 \
    && rm -rf /var/lib/apt/lists/* /tmp/*

#ARG UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    appuser

USER appuser

COPY --from=builder /app/target/${TARGET_PROFILE}/main /bin/
COPY --from=builder /app/public /bin/public

# Expose the port that the application listens on.
EXPOSE 8000

# What the container should run when it is started.
ENTRYPOINT [ "main" ]