FROM rust:1.43.0-slim as build

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

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
      /target/release/mia_template_service_name_placeholder \
      /usr/local/bin/

CMD ["mia_template_service_name_placeholder"]
