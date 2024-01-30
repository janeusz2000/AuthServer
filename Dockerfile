# Use the official Rust image as the base image
FROM rust:latest as build

# Set the working directory
WORKDIR /usr/src/auth_server

# Copy the entire project into the Docker image
COPY . .

# Install necessary dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Udpate the dependencies
RUN cargo update 

# Build the authentication server
RUN cargo build --release

# Create a new stage for the runtime image
FROM ubuntu:latest

# Copy the authentication server binary from the build stage
COPY --from=build /usr/src/auth_server/target/release/auth_server /usr/local/bin/auth_server

# Set the environemnt varables for adress and port
ENV AUTH_SERVER_ADDRESS localhost
ENV AUTH_SERVER_PORT 8080
ENV DATABASE_ADDRESS localhost
ENV DATABASE_USER root
ENV DATABASE_PASSWORD password
ENV DATABASE_PORT 3306
ENV DATABASE_NAME user_database

# Expose the port the authentication server will run on
EXPOSE 8080

# Run the authentication server
CMD ["auth_server"]
