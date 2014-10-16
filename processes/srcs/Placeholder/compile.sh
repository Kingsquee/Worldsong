#!/bin/bash
echo "" &&
echo "Building PlaceholderProcess" &&
rustc --crate-type=dylib -L ../../../dataset/target/ -L ../../../dataset/target/deps -L ../../../dataset/target/native/* --out-dir ../../targets/ process.rs &&
echo "Done"