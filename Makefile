BATCH_COMMAND := sync-token

build:
	cargo install cross
	cross build --target x86_64-unknown-linux-musl --release

deploy:
	sam build
	sam deploy --no-confirm-changeset --no-fail-on-empty-changeset

build-ApiFunction: target/x86_64-unknown-linux-musl/release/api
	cp $< $(ARTIFACTS_DIR)/bootstrap

build-SubscriberFunction: target/x86_64-unknown-linux-musl/release/subscriber
	cp $< $(ARTIFACTS_DIR)/bootstrap

build-BatchFunction: target/x86_64-unknown-linux-musl/release/batch
	cp $< $(ARTIFACTS_DIR)/bootstrap

run-local-api:
	SSM_DOTENV_PARAMETER_NAME=/canvas-nft-generator/server/dotenv cargo run --bin api

run-local-batch:
	SSM_DOTENV_PARAMETER_NAME=/canvas-nft-generator/server/dotenv COMMAND='{"command":"${BATCH_COMMAND}"}' cargo run --bin batch

run-batch:
	aws lambda invoke \
		--function-name canvas-nft-generator-BatchFunction-naA5F8R3IDum \
		--payload '{"command":"${BATCH_COMMAND}"}' \
		--cli-binary-format raw-in-base64-out \
		/dev/null


extract-abi:
	cat ethereum/artifacts/contracts/Canvas.sol/Canvas.json | jq '.abi' | jq -c | jq '@json' | cat