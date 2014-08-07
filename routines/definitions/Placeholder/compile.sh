#!/bin/bash
rustc --crate-type=dylib -L ../../../databases/binaries/ -o ../../binaries/${PWD##*/}.routine routine.rs