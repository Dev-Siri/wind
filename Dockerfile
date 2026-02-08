FROM rust:1.91-slim-bookworm AS prebuild

WORKDIR /usr/src/wind

# Wind specific files
COPY . .

FROM prebuild AS builder

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/wind/target/release/wind /usr/local/bin/wind
COPY --from=prebuild /usr/src/wind/wind.json /usr/src/wind/rules.json /usr/local/bin/

WORKDIR /usr/local/bin

CMD [ "wind" ]
