#Build stage
FROM rust:1.78-slim AS builder

# Install python and venv
RUN apt-get update && apt-get install -y \
    python3 \
    python3-venv \
    && rm -rf /var/lib/apt/lists/*

#create venv and set env path
RUN python3 -m venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"

WORKDIR /usr/src/app
COPY . .

# Install Python dependencies in venv
RUN pip install --no-cache-dir transformers torch sentencepiece sacremoses

# Build Rust project
RUN cargo build --release

#Final image
FROM debian:bookworm-slim

# Install python and venv
RUN apt-get update && apt-get install -y \
    python3 \
    python3-venv \
    && rm -rf /var/lib/apt/lists/*

#copy builder venv and ser env path
COPY --from=builder /opt/venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"
ENV IN_DOCKER=TRUE

#Rust binary copy
COPY --from=builder /usr/src/app/target/release/rust_project /usr/local/bin/rust_project
#Python code copy
COPY src/translation.py /usr/local/bin/translation.py

# Set the command to run
ENTRYPOINT ["rust_project"]