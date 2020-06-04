FROM rust:1.43.0-slim as build

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN mkdir src

RUN echo "fn main() {panic!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/rust_hello_world*
RUN rm -f target/release/incremental/rust_hello_world*
RUN rm -f target/release/deps/rust_hello_world*

COPY . .

RUN cargo build --release

# ==================================

FROM gcr.io/distroless/cc

LABEL maintainer="%CUSTOM_PLUGIN_CREATOR_USERNAME%" \
      name="mia_template_service_name_placeholder" \
      description="%CUSTOM_PLUGIN_SERVICE_DESCRIPTION%" \
      eu.mia-platform.url="https://www.mia-platform.eu" \
      eu.mia-platform.version="0.1.0" \
      eu.mia-platform.language="rust" \
      eu.mia-platform.framework="Rust"

COPY --from=build \
      /target/release/rust-hello-world \
      /usr/local/bin/

CMD ["rust-hello-world"]
