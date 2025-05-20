.PHONY: build run clean docker-build docker-run

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
	docker run --rm $(PROJECT_NAME) $1 $2 $3


# Run tests
test:
	cargo test

# Check code formatting
fmt:
	cargo fmt -- --check

# Format code
fmt-fix:
	cargo fmt

run_sample:
	rm -rf output
	cargo run -- /Users/lukasgarbacik/Desktop/rust_project/input_sample /Users/lukasgarbacik/Desktop/rust_project/output "es"


docker_sample: docker-build
	docker run --rm -it \
	  -v /Users/lukasgarbacik/Desktop/rust_project/input_sample:/input \
	  -v /Users/lukasgarbacik/Desktop/rust_project/output:/output \
	  $(PROJECT_NAME) /input /output es