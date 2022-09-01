# kvdynamodb-provider Makefile

CAPABILITY_ID = "aws:kvdynamodb"
NAME = "kvdynamodb-provider"
VENDOR = "com.mattgilbride"
PROJECT = kvdynamodb_provider
VERSION = 0.1.0
REVISION = 0

include ./provider.mk

delete-test-table:
	aws dynamodb delete-table \
		--table-name kvdynamodb \
		--endpoint-url http://localhost:8000


create-test-table:
	aws dynamodb create-table \
    	--table-name kvdynamodb \
    	--attribute-definitions AttributeName=key,AttributeType=S \
    	--key-schema AttributeName=key,KeyType=HASH \
    	--provisioned-throughput ReadCapacityUnits=1,WriteCapacityUnits=1 \
		--endpoint-url http://localhost:8000
	aws dynamodb update-time-to-live \
		--table-name kvdynamodb \
		--time-to-live-specification "Enabled=true, AttributeName=ttl" \
		--endpoint-url http://localhost:8000

test:: export AWS_DYNAMODB_LOCAL_URI=http://localhost:8000
test:: export TABLE_NAME=kvdynamodb
test:: export KEY_ATTRIBUTE=key
test:: export VALUE_ATTRIBUTE=value
test:: export TTL_ATTRIBUTE=ttl
test::
	-ps -ax | grep -i kvdynamodb_provider | awk '{print $$1}' | xargs kill -9
	docker-compose down
	docker-compose up -d
	${MAKE} create-test-table
	cargo clippy --all-targets --all-features
	RUST_BACKTRACE=1 cargo test -- --nocapture

push-gh-arm:
	wash reg push -u matthewtgilbride -p $$GH_PERSONAL_ACCESS_TOKEN \
    ghcr.io/matthewtgilbride/kvdynamodb_provider_arm:0.1.0 \
    build/kvdynamodb_provider.par.gz

push-gh-x86:
	wash reg push -u matthewtgilbride -p $$GH_PERSONAL_ACCESS_TOKEN \
    ghcr.io/matthewtgilbride/kvdynamodb_provider_x86:0.1.0 \
    build/kvdynamodb_provider.par.gz

