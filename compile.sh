#!/bin/bash
# get current path
path="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )" &&
cd $path &&
echo "" &&
echo "Compiling Worldsong Suite" &&
echo "" &&

# compiling common
cd ./common/ &&
cargo build &&

cd $path &&

# compiling kernel
cd ./kernel/ &&
cargo build &&

cd $path &&

echo "" &&
echo "Compiling Processes" &&
cd ./schedules/ &&
find ./*/*/ -name 'Cargo.toml' -execdir cargo build \;
echo "" &&
echo "Worldsong Compiled Successfully!"
