# Base image
FROM rust:latest as builder
RUN apt-get update -qq && apt-get upgrade -qq && apt-get install -qq \
    bash \
    g++ \
    ca-certificates \
    musl-dev \
    libssl-dev \
    make \
    python3 \
    gcc \
    zlib1g-dev \
    libc-dev \
    python3-setuptools \
    cmake

# Set the working directory
WORKDIR /app/rust-service
# Copy the Cargo.toml and Cargo.lock files to the working directory
COPY Cargo.toml Cargo.lock ./
# Copy the rest of the source code to the working directory
ADD ./src ./src
# Build the app
RUN cargo build --release
# Specify the command to run your app
CMD [ "cargo", "run","--release"]

