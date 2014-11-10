#!/bin/bash
# get current path
path="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )" &&
cd $path &&
cargo build
