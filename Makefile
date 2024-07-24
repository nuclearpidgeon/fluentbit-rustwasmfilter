.PHONY: build test

DOCKER_IMAGE_NAME=fluent/fluent-bit:latest

test_deps:
# this if is a makefile operation that will be performed at makefile-parse-time
ifeq ("$(shell docker images -q ${DOCKER_IMAGE_NAME} 2> /dev/null)","")
	docker pull ${DOCKER_IMAGE_NAME}
endif

build:
	cargo build

test_json: test_deps build
	docker run --rm \
		--mount type=bind,source=$(shell pwd)/target/wasm32-unknown-unknown/debug,target=/build_out \
		${DOCKER_IMAGE_NAME} \
		/opt/fluent-bit/bin/fluent-bit \
			-i dummy \
			-F wasm -p event_format=json -p wasm_path=/build_out/fluentbit_rustwasmfilter.wasm -p function_name=hello_world__json -m '*' \
			-o stdout -m '*'
test_msgpack: test_deps build
	docker run --rm \
		--mount type=bind,source=$(shell pwd)/target/wasm32-unknown-unknown/debug,target=/build_out \
		${DOCKER_IMAGE_NAME} \
		/opt/fluent-bit/bin/fluent-bit \
			-i dummy \
			-F wasm -p event_format=msgpack -p wasm_path=/build_out/fluentbit_rustwasmfilter.wasm -p function_name=hello_world__msgpack -m '*' \
			-o stdout -m '*'