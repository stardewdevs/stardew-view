FROM rust:1.78-bookworm AS builder

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
    pkg-config \
    libxkbcommon-dev \
    libwayland-dev \
    libfontconfig1-dev \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
    libxkbcommon0 \
    libwayland-client0 \
    libfontconfig1 \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/libstardew_view.so /usr/local/lib/
COPY --from=builder /app/target/release/examples/ /usr/local/bin/

ENV LD_LIBRARY_PATH=/usr/local/lib
CMD ["sh"]