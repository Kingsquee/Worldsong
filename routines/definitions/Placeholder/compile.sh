#!/bin/bash
rustc --crate-type=dylib -o ../../binaries/${PWD##*/}.routine routine.rs