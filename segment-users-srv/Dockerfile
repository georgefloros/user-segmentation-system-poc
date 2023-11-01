# Base image
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to the working directory
COPY Cargo.toml Cargo.lock ./

# Build the dependencies
RUN cargo build --release

# Copy the rest of the source code to the working directory
COPY . .

# Build the app
RUN cargo build --release

# Specify the command to run your app
CMD ["cargo", "run", "--release"]