# syntax=docker/dockerfile:1
#
# Build from the repository root (the service shares the Cargo workspace
# with the analyzer whose types it reuses):
#
#   docker build -f service/Dockerfile -t pr-hygiene-service .
#
# For deployment, pin both base images by digest.

FROM rust:1-bookworm AS build
WORKDIR /src
# Only what the build needs; see Dockerfile.dockerignore. rust-toolchain.toml
# is left out so the image's own stable toolchain builds it, rather than
# rustup downloading another.
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY service ./service
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    cargo build --release --locked -p pr-hygiene-service \
    && install -D target/release/pr-hygiene-service /out/pr-hygiene-service \
    && install -d -o 65532 -g 65532 /out/data

# glibc and libgcc only; no shell, no package manager. Runs as uid 65532.
FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=build /out/pr-hygiene-service /usr/local/bin/pr-hygiene-service
COPY --from=build --chown=65532:65532 /out/data /data
# The page: its own files only — no fixtures, no check script.
COPY site/index.html site/app.js site/style.css /srv/site/
COPY site/js /srv/site/js
COPY site/vendor/d3.min.js site/vendor/plot.umd.min.js /srv/site/vendor/
ENV PR_HYGIENE_BIND=0.0.0.0:8080 \
    PR_HYGIENE_DB=/data/pr-hygiene.sqlite3 \
    PR_HYGIENE_SITE_DIR=/srv/site
# The database lives here: a local volume, not a network file system.
VOLUME ["/data"]
EXPOSE 8080
USER 65532:65532
ENTRYPOINT ["/usr/local/bin/pr-hygiene-service"]
CMD ["serve"]
