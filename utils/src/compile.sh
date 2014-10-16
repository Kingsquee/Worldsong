#!/bin/bash
#rustc --crate-type=dylib -L ../../binaries --out-dir ../../binaries/ lib.rs
echo "" &&
echo "Building Utils" &&
cargo build &&
echo "Done"