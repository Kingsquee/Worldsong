#!/bin/bash
# Generic script for compiling processes

# get current path
path="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )" &&
# delete from last / to end
dir=${path%/*} &&
# delete from beginning to last /
finalName=${dir##*/} &&

cd $path &&

echo "Compiling $finalName process" &&
mkdir -p ./../target &&
rustc -L ./../../../common/target -L ./../../../common/target/deps -L ./../../../common/target/native --out-dir ./../target --crate-type="dylib" graphics.rs &&
echo "Done"
