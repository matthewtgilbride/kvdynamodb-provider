# kvdynamodb-provider Makefile

CAPABILITY_ID = "aws:kvdynamodb"
NAME = "kvdynamodb-provider"
VENDOR = "com.mattgilbride"
PROJECT = kvdynamodb_provider
VERSION = 0.1.0
REVISION = 0

include ./provider.mk

test::
	cargo clippy --all-targets --all-features

