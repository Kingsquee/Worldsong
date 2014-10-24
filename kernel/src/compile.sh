#!/bin/bash
mkdir -p ./../target;
rustc -L ./../../common/target -L ./../../common/target/deps -L ./../../common/target/native --out-dir ./../target kernel.rs;