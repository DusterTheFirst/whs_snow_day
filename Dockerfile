# THe builder container
FROM rust:alpine as builder
WORKDIR /app

# Make sure all dependencies are installed
RUN apk update && apk add musl-dev libressl-dev pkgconf

# Copy source code over
COPY . .
# Compile and link the app
RUN RUSTFLAGS="-C target-feature=-crt-static" cargo install --path .

# The app container
FROM alpine:latest
LABEL Author="Zachary Kohnen"

# Copy binary to the app
COPY --from=builder /usr/local/cargo/bin/whs_urgent_alerts /usr/local/bin/whs_urgent_alerts

# Copy configs and other files needed for the app
COPY Config.toml ./data
COPY .release.env .
COPY preposts.json ./data

# Expose the config and preposts to the outside
VOLUME [ "/data" ]

# Run the app
CMD ["whs_urgent_alerts"]