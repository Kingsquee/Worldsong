#!/bin/bash
rustc --crate-type=dylib -L ../../../databases/binaries/ -L ../../../utils/ -o ../../binaries/${PWD##*/}.routine routine.rs