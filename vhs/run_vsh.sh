#!/bin/bash

# Create alias for univiz pointing to target/debug
export PATH="$(realpath "$(dirname "$0")/../target/debug"):$PATH"

# Change to current directory (vhs folder)
cd "$(dirname "$0")"

# Run evp demo.tape
evp demo.tape
