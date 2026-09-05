# ── Build stage ──────────────────────────────────────────────────────────────
FROM rust:1-slim AS builder

WORKDIR /app

# `libxml` links system libxml2 rather than using a pure-Rust equivalent.
RUN apt-get update \
    && apt-get install -y --no-install-recommends libxml2-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Manifests and build inputs first, so the dependency layer is cached and only
# rebuilt when they actually change.
COPY Cargo.toml Cargo.lock* build.rs ./
COPY proto/ ./proto/

# Build a throwaway binary to compile and cache the dependency graph.
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs \
    && echo '' > src/lib.rs \
    && cargo build --release 2>/dev/null; \
    rm -rf src target/release/iso20022d target/release/deps/iso20022-*

COPY src/ ./src/
COPY db/ ./db/
RUN cargo build --release

# ── Runtime stage ─────────────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libxml2 \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -r appuser && useradd -r -g appuser appuser

WORKDIR /app

COPY --from=builder /app/target/release/iso20022d ./iso20022d
COPY --from=builder /app/db/ ./db/

# The XSD set this service validates against is NOT in this repository; it is
# read from ./xsd_to_rs/xsds at startup. Mount it, e.g.:
#   docker run -v /path/to/xsds:/app/xsd_to_rs/xsds ...
VOLUME ["/app/xsd_to_rs/xsds"]

RUN chown -R appuser:appuser /app
USER appuser

ENV RUST_LOG=info

EXPOSE 3000

# Configuration is entirely environment-based - see the README.
ENTRYPOINT ["./iso20022d"]
