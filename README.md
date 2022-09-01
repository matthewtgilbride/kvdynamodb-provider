# [wasmcloud](https://wasmcloud.com/) kvdynamodb capability provider

This [capability provider](https://wasmcloud.dev/reference/host-runtime/capabilities/)
implements the [aws:kvdynamodb](https://github.com/matthewtgilbride/kvdynamodb) capability.

It expects AWS credentials to be available via standard mechanisms
(instance profile, credential file, environment variables, etc).

It also expects the following values to be provided via `config_json` or environment variables:
 - TABLE_NAME: the name of the DynamoDB table
 - KEY_ATTRIBUTE: the string attribute to be used for keys
 - VALUE_ATTRIBUTE: the string attribute to be used for values
 - TTL_ATTRIBUTE: (Optional) the numeric attribute to be used for time-to-live

Build with 'make'. Test with 'make test'.
