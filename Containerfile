FROM rust:1.94-slim

RUN rustup component add llvm-tools-preview && \
    cargo install cargo-llvm-cov

WORKDIR /app

CMD ["cargo", "llvm-cov", "--html"]
