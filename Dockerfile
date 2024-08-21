# Stage 1: Build the application
FROM rust:1.70 as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock (if available)
COPY Cargo.toml Cargo.lock ./

# Pre-build dependencies for faster build times
RUN cargo build --release || true

# Copy the source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create a minimal image for running the application
FROM debian:buster-slim

# Set the working directory
WORKDIR /usr/src/app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/webhook_receiver .

# Expose the port on which the app will run
EXPOSE 8080

# Command to run the compiled binary
CMD ["./webhook_receiver"]