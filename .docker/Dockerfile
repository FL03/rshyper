# Author: FL03 <jo3mccain@icloud.com>
ARG RUST_VERSION=latest

FROM rust:${RUST_VERSION} AS builder-base
# update and upgrade the system
RUN apt-get update -y && \
    apt-get upgrade -y
# STAGE 2: build
FROM builder-base AS builder
# declare some environment variables
ENV RUST_BACKTRACE=1 \
    CARGO_NET_GIT_FETCH_WITH_CLI=true \
    CARGO_HOME=/usr/local/cargo
# setup the working directory
WORKDIR /app
# copy the source code
ADD . .
# build the project
RUN --mount=type=cache,target=/workspace/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release --features full --workspace
# STAGE 3: create the runtime image
FROM debian:bookworm-slim AS runner-base
# update and upgrade the system packages
RUN apt-get update -y && \
    apt-get upgrade -y
# create a group and add a non-root user
RUN groupadd -g 10001 agroup && \
    useradd -m -u 10001 -g agroup auser
# switch to the new user
USER auser
# copy the executable(s) to the system
COPY --from=builder --chown=auser:agroup /app/target/release/pzzld /usr/local/bin/pzzld
# copy the configuration directory
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/.config /opt/pzzld/.config
# copy any additional configuration files
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/*.config.toml* /opt/pzzld/.config/*.config.toml*
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/Puzzled.toml* /opt/pzzld/.config/Puzzled.toml*
# set the permissions
RUN chmod +x /usr/local/bin/pzzld && \
    chmod +x /opt/pzzld/.config && \
    chown auser /usr/local/bin/pzzld && \
    chown -R auser /opt/pzzld
# STAGE 4: use the configured runtime base to create the final image
FROM runner-base AS runner
# Set the environment variables
ENV APP_MODE=release \
    PZZLD_HOST="0.0.0.0" \
    PZZLD_PORT=8080 \
    PZZLD_CONFIG="/opt/pzzld/.config" \
    PZZLD_WORKDIR="/opt/pzzld" \
    RUST_LOG="pzzld=debug,info"
# set the working directory
WORKDIR /opt/pzzld
# expose the port
EXPOSE ${PZZLD_PORT}
# set the entrypoint
ENTRYPOINT ["pzzld"]
