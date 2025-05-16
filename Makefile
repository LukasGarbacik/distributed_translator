.PHONY: build run clean docker-build docker-run

# Project name (change if different from directory name)
PROJECT_NAME := rust_project

# Build the project
build:
	cargo build

# Build for release
release:
	cargo build --release

# Run the project
run:
	cargo run

# Clean the project
clean:
	cargo clean

# Build Docker image
docker-build:
	docker build -t $(PROJECT_NAME) .

# Run Docker container
docker-run:
	docker run --rm $(PROJECT_NAME)

# Build and run in Docker
docker: docker-build docker-run

# Run tests
test:
	cargo test

# Check code formatting
fmt:
	cargo fmt -- --check

# Format code
fmt-fix:
	cargo fmt

# Run clippy for linting
lint:
	cargo clippy -- -D warnings

run_sample:
	cargo run -- /Users/lukasgarbacik/Desktop/rust_project/input_sample /Users/lukasgarbacik/Desktop/rust_project/output