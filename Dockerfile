FROM rust:1.87.0 AS chef

RUN apt-get update \
    && apt-get install -y --no-install-recommends build-essential ca-certificates cmake gcc libc6-dev pkg-config libprotobuf-dev protobuf-compiler curl wget libsqlite3-dev sqlite3 \
    && rm -rf /var/lib/apt/lists/* \
    && curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

RUN mkdir /usr/local/nvm
ENV NVM_DIR=/usr/local/nvm
ENV NODE_VERSION=22.16.0
RUN curl https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash \
    && . $NVM_DIR/nvm.sh \
    && nvm install $NODE_VERSION \
    && nvm alias default $NODE_VERSION \
    && nvm use default

ENV NODE_PATH=$NVM_DIR/v$NODE_VERSION/lib/node_modules
ENV PATH=$NVM_DIR/versions/node/v$NODE_VERSION/bin:$PATH

# RUN wget -qO- https://get.pnpm.io/install.sh | ENV="$HOME/.shrc" SHELL="$(which sh)" sh -
RUN npm install -g pnpm@latest-10

# We only pay the installation cost once,
# it will be cached from the second build onwards
RUN cargo install cargo-chef
RUN cargo install diesel_cli --no-default-features --features sqlite
# RUN cargo binstall diesel_cli
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

RUN cd warhundred-fe \
    && pnpm install \
    && pnpm run build

FROM debian:stable-slim

WORKDIR /app

ARG TARGET_PROFILE
ENV TARGET_PROFILE=${TARGET_PROFILE}

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libsqlite3-dev sqlite3 libpq5 \
    && rm -rf /var/lib/apt/lists/* /tmp/*

# Set DATABASE_URL for migrations
ENV DATABASE_URL=/app/database.db

#ARG UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    appuser

USER root

COPY --from=builder /app/target/${TARGET_PROFILE}/main /app/
COPY --from=builder /app/public /app/public
COPY --from=builder /usr/local/cargo/bin/diesel /bin/

COPY src/app/migrations /app/migrations
COPY database.db /app/database.db
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Ensure appuser has permissions to run migrations
RUN mkdir -p /app && chown -R appuser:appuser /app && chmod -R 755 /app && \
    chmod 644 /app/database.db

USER appuser

# Expose the port that the application listens on.
EXPOSE 8000

# Use entrypoint script to run the binary
ENTRYPOINT ["/entrypoint.sh"]

CMD ["./app/main"]