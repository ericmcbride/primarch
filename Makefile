IMAGE_NAME = "ericmcbridedeveloper/primarch"
GIT_HASH = $(shell git rev-parse HEAD)

tag:
	docker build -t ${IMAGE_NAME}:ci-${GIT_HASH} .

push:
	docker push ${IMAGE_NAME}:ci-${GIT_HASH}

build:
	cargo build

format:
	cargo fmt --all

check:
	cargo check
