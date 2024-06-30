# Build and Run stage
FROM rust:1.79-buster

WORKDIR /app

# accept the build argument
ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

# Copy the source code
COPY . .

# Install cargo-watch for hot reloading
RUN cargo install cargo-watch

# Command to run the application with hot reload
CMD ["cargo", "watch", "-x", "run"]