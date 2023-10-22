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

.PHONY: repl_origin
repl_origin:
	cargo run --bin runtime --features="binaries"

.PHONY: repl
repl:
	(cd three_body_e2021 && cargo run --features="repl")

.PHONY: example
example:
	./target/debug/runtime ./example/乱纪元.3body
	./target/debug/runtime ./example/地球.3body