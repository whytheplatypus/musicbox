PROJECT_DIR=$(shell pwd)

build: target/arm-unknown-linux-gnueabihf/release/musicbox

buildall: contrib/image.id
	docker run \
		--rm \
		-v $(PROJECT_DIR)/target:/build \
		-v $(PROJECT_DIR):/src \
		musicbox-cross:latest
	sudo chown -R $(shell id -nu):$(shell id -ng) target

target/arm-unknown-linux-gnueabihf/release/musicbox: src/main.rs contrib/image.id
	docker run \
		--rm \
		-v $(PROJECT_DIR)/target:/build \
		-v $(PROJECT_DIR):/src \
		musicbox-cross:latest \
		cargo build --release --target arm-unknown-linux-gnueabihf --no-default-features 
	sudo chown -R $(shell id -nu):$(shell id -ng) target


contrib/image.id: contrib/Dockerfile contrib/docker-build.sh
	docker build \
		--iidfile contrib/image.id \
		-t musicbox-cross \
		-f contrib/Dockerfile .

clean:
	sudo chown -R $(shell id -nu):$(shell id -ng) target
	cargo clean;
	rm contrib/image.id;
