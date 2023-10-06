#!/usr/bin/env bats

@test "Add default annotations to ingress" {
	run kwctl run  --request-path test_data/ingress_creation.json  annotated-policy.wasm
	[ "$status" -eq 0 ]
	echo "$output"
}
