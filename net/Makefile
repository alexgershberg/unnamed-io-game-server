all: rs-build ts-build

rs-build:
	@cargo build

.FORCE ts-build:
	@wasm-pack build --target bundler