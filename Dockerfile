# Install Rust
FROM rust:latest

# Install Elm
WORKDIR /usr/local/bin
RUN ["curl", "-L", "-o", "elm.gz", "https://github.com/elm/compiler/releases/download/0.19.1/binary-for-linux-64-bit.gz"]
RUN ["gzip", "-d", "elm.gz"]
RUN ["chmod", "+x", "elm"]

# Install node (for elm repl)
FROM node:14
