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
		--endpoint-url http://localhost:8000 \
		--table-name kvdynamodb \


create-test-table:
	aws dynamodb create-table \
		--endpoint-url http://localhost:8000 \
    	--table-name kvdynamodb \
    	--attribute-definitions AttributeName=K,AttributeType=S \
    	--key-schema AttributeName=K,KeyType=HASH \
    	--provisioned-throughput ReadCapacityUnits=1,WriteCapacityUnits=1

test:: export TABLE_NAME=kvdynamodb
test:: export KEY_ATTRIBUTE=K
test:: export VALUE_ATTRIBUTE=V
test:: export AWS_DYNAMODB_LOCAL_URI=http://localhost:8000
test::
	docker-compose down
	docker-compose up -d
	${MAKE} create-test-table
	cargo clippy --all-targets --all-features
	RUST_BACKTRACE=1 cargo test -- --nocapture

