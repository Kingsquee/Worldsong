#!/bin/bash
rustc --crate-type=dylib -o ../../binaries/${PWD##*/} verse.rs