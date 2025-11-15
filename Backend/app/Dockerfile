# Multi-stage build for Rust application
FROM rust:1.75 as builder

# Set working directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false appuser

# Set working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/bisca /app/bisca

# Copy config file if it exists
COPY --chown=appuser:appuser config.example.toml ./config.toml

# Create logs directory
RUN mkdir -p /app/logs

# Change ownership to app user
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 3000

# Set environment variables
ENV RUST_LOG=info
ENV HOST=0.0.0.0
ENV PORT=3000

# Run the application
CMD ["./bisca"]