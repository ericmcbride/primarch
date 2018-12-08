IMAGE_NAME = "ericmcbridedeveloper/primarch"
GIT_HASH = $(shell git rev-parse HEAD)

tag:
	docker build -t ${IMAGE_NAME}:ci-${GIT_HASH} -t latest .
push:
	docker push ${IMAGE_NAME}:ci-${GIT_HASH} latest
build:
	cargo build

format:
	cargo fmt --all

check:
	cargo check
