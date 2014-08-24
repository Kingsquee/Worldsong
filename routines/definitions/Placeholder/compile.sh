#!/bin/bash
rustc --crate-type=dylib -L ../../../databases/binaries/ -L ../../../utils/binaries/ -o ../../binaries/${PWD##*/}.routine routine.rs