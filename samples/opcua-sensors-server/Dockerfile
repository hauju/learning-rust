# Build project
FROM rust:latest AS build-env
WORKDIR /app
COPY . .
RUN cargo build --release

# Build runtime image
FROM debian:buster-slim
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends libssl-dev
WORKDIR /app
COPY --from=build-env /app/target/release/opcua-sensors-server ./opcua-sensors-server
COPY --from=build-env /app/server.conf ./server.conf
ENTRYPOINT ["./opcua-sensors-server"]