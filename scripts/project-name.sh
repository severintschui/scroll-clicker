#!/bin/bash

# get project name from Cargo.toml file
grep '^name =' "$(git rev-parse --show-toplevel)/Cargo.toml" | awk -F'"' '{print $2}'