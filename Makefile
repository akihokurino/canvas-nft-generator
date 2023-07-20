build:
	cargo install cross
	cross build --target x86_64-unknown-linux-musl --release

deploy:
	sam build
	sam deploy --no-confirm-changeset --no-fail-on-empty-changeset

build-ApiFunction: target/x86_64-unknown-linux-musl/release/api
	cp $< $(ARTIFACTS_DIR)/bootstrap

run-local-api:
	SSM_DOTENV_PARAMETER_NAME=/canvas-nft-generator/server/dotenv cargo run --bin api