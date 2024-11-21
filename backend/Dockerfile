# Use an official Rust image as the base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy only the Cargo.toml and Cargo.lock first to leverage Docker's layer caching
COPY Cargo.toml Cargo.lock ./

# Cache dependencies
RUN cargo fetch

# Copy the source code
COPY .. .

# Install cargo-watch for automatic reloading (optional for hot-reloading)
RUN cargo install cargo-watch

# Expose your application port (e.g., 8080)
EXPOSE 8080

# Command for debug mode (cargo run)
CMD ["cargo", "watch", "-x", "run"]
