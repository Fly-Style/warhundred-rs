FROM rust:1.87.0 AS chef

RUN apt-get update \
    && apt-get install -y --no-install-recommends build-essential ca-certificates cmake gcc libc6-dev pkg-config libprotobuf-dev protobuf-compiler curl wget \
    && rm -rf /var/lib/apt/lists/*

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
COPY --from=builder /app/public /app/public

# Expose the port that the application listens on.
EXPOSE 8000

# What the container should run when it is started.
ENTRYPOINT [ "main" ]