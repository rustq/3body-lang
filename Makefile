.PHONY: setup
setup:
	(cd web && yarn)

.PHONY: start
start:
	make build_wasm
	(cd web && yarn start)

.PHONY: test
test:
	cargo test

.PHONY: build_repl
build_repl:
	cargo build --release

.PHONY: build_wasm
build_wasm:
	cargo build --bin wasm --release --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/wasm.wasm web/src/monkey.wasm

.PHONY: web_deploy
web_deploy:
	make build_wasm
	(cd web && yarn --pure-lockfile && yarn deploy)

.PHONY: repl
repl:
	cargo run --bin runtime --features="binaries"

.PHONY: example
example:
	./target/debug/runtime ./example/乱纪元.3body
	./target/debug/runtime ./example/地球.3body